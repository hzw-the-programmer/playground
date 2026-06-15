# Tokio 多线程调度器 本地任务队列（Work-Steal 就绪队列） 源码全解析

这份代码是 Tokio 多线程 runtime 核心：每个 Worker 线程专属的本地运行队列，实现单生产者、多消费者（任务窃取） 并发队列，支撑工作窃取调度算法。

整体定位：

- 每个 Worker 持有一个 `Local`（唯一生产者，仅本线程 push/pop）
- 其他 Worker 通过 `Steal` 句柄（多消费者）窃取任务
- 环形固定大小队列 + 原子双表头 + 无锁设计 + 溢出分流策略

---

# 一、整体架构 & 核心概念梳理

## 1. 核心角色分工

结构体	角色	访问权限
`Local<T>`	本地生产者	单线程独占：仅归属 Worker 本线程使用，负责入队、本地出队
`Steal<T>`	远程窃取者	多线程共享：其他 Worker 用来偷任务
`Inner<T>`	队列共享内核	`Arc` 跨线程共享，存放原子索引 + 任务缓冲区

队列属性：

- 固定容量：正常环境 `256`，Loom 测试环境 `4`
- 环形缓冲区：`index = pos & MASK`，`MASK = cap - 1`
- 索引使用无符号环绕运算 `wrapping_add`/`wrapping_sub`
- 无锁并发：靠 `Atomic` + CAS 实现，全程无互斥锁

---

## 2. 类型别名（平台兼容 + 抗 ABA）

```rust
cfg_has_atomic_u64! {
    type UnsignedShort = u32;
    type UnsignedLong = u64;
}
cfg_not_has_atomic_u64! {
    type UnsignedShort = u16;
    type UnsignedLong = u32;
}
```

- 设计目的：加宽索引位数，缓解 ABA 问题（issue #5041）
- `UnsignedLong`：拼接两个 `UnsignedShort`，作为 `head` 原子变量底层类型
- `AtomicUnsignedLong`：打包两个表头进一个原子变量，保证读写原子性

---

# 二、`Inner<T>` 队列内核详解（最核心）

```rust
pub(crate) struct Inner<T: 'static> {
    head: AtomicUnsignedLong,
    tail: AtomicUnsignedShort,
    buffer: Box<[UnsafeCell<MaybeUninit<task::Notified<T>>>; LOCAL_QUEUE_CAPACITY]>,
}
```

## 1. 字段逐解

### (1) `head: AtomicUnsignedLong` 复合表头（精髓）

一个原子变量同时存两个索引：

- 低半部分 (L)：`real` → 真实队头，本地生产者 `pop` 使用
- 高半部分 (H)：`steal` → 窃取标记头，标记「正在被其他线程窃取的起始位置」

规则：

- `steal == real`：当前无线程在窃取，队列正常
- `steal != real`：有线程正在批量窃取，其他窃取者暂时不能操作队列
- 用一个原子类型承载双索引：保证读取 / 修改二者是原子操作，避免数据撕裂

配套工具函数：

- `pack(steal, real)`：把两个短索引拼接成一个长整数存入 `head`
- `unpack(n)`：把长整数拆分成 `(steal, real)` 两个索引

### (2) `tail: AtomicUnsignedShort` 队尾

- 唯一写者：仅本 Worker 生产者线程修改
- 多读者：所有窃取线程都可以读取
- 记录下一个入队位置

### (3) buffer 环形缓冲区

```rust
Box<[UnsafeCell<MaybeUninit<task::Notified<T>>>; LOCAL_QUEUE_CAPACITY]>
```

逐层拆解：

- `[...; N]`：编译期定长数组，固定容量环形队列
- `MaybeUninit`：内存先分配、延迟初始化，队列空时元素是未初始化状态
- `UnsafeCell`：内部可变性，绕过 Rust 借用规则，实现同一内存被生产者 / 窃取者读写
- `task::Notified<T>`：Tokio 可被唤醒的异步任务

安全前提：通过 `head`/`tail` 索引逻辑保证同一位置不会被读写并发冲突。

## 2. 安全实现

```rust
unsafe impl<T> Send for Inner<T> {}
unsafe impl<T> Sync for Inner<T> {}
```

手动实现 `Send`/`Sync`：

开发者通过原子索引 + 访问规则保证并发安全，编译器无法自动推导，所以手动 `unsafe impl`。

---

# 三、工具函数 & 基础常量

## 1. `make_fixed_size` 动态切片 → 定长数组

- 把 `Box<[T]>` 动态切片转为 `Box<[T; N]>` 定长数组
- 运行时断言长度一致 + `unsafe` 裸指针强转，零拷贝
- 用途：初始化固定大小任务缓冲区

## 2. 队列长度计算 `len()`

```rust
fn len(head: UnsignedShort, tail: UnsignedShort) -> usize {
    tail.wrapping_sub(head) as usize
}
```

环形队列标准算法：

利用无符号环绕减法计算元素个数，天然适配环形溢出场景。

## 3. `pack` / `unpack` 索引拼接 / 拆分

以平台支持 `u64`/`u32` 举例：

- `pack`：`steal` 左移 32 位 → 高半区；`real` 放低半区，按位或合并
- `unpack`：按位与取低半区 = `real`；右移 32 位 = `steal`

核心设计：

两个索引捆绑在一个原子变量，一次原子读写同时拿到两个状态，彻底避免部分更新导致的竞态。

---

# 四、队列创建 `local()` 函数

```rust
pub(crate) fn local<T: 'static>() -> (Steal<T>, Local<T>)
```

流程：

1. 创建 `Vec` 并填充 `UnsafeCell<MaybeUninit>` 未初始化槽位
2. 调用 `make_fixed_size` 转为定长数组缓冲区
3. 初始化 `head=0`、`tail=0`，队列为空
4. 用 `Arc` 包裹 `Inner`，生成一对句柄：
   - `Local`：本线程生产者
   - `Steal`：多线程窃取者（可 Clone）
5. 典型使用：每个 Worker 启动时调用一次，持有专属本地队列。

---

# 五、`Local<T>` 生产者侧 API（本线程操作）

`Local` 规则：只有归属 Worker 线程能调用，单生产者。

## 1. 基础状态查询

- `len()`：队列当前任务数，用 `real head` + `tail` 计算
- `remaining_slots()`：剩余空位，用 `steal head` 计算（考虑正在窃取的区间）
- `has_tasks()`：判断队列非空

关键区别：

- 计算队列总元素：用 `real`（真实队头）
- 计算可用空位：用 `steal`（窃取标记头），避免和窃取线程冲突

## 2. 批量入队 `push_back`

批量推送一批任务，要求必须能容纳，否则 panic：

1. 加载 `head` 拆出 `steal` 索引
2. 无同步读取 `tail`（本线程专属，无竞争）
3. 校验剩余空间足够，否则 panic
4. 遍历任务：按 `tail & MASK` 取槽位，`ptr::write` 写入任务
5. `tail` 环绕递增，最后原子 `store` 发布

## 3. 普通入队（带溢出）`push_back_or_overflow`

生产环境核心入队逻辑，队列满时触发溢出分流：

整体流程

1. 循环加载 `head`/`tail` 判断容量
2. 分支：
   - 有空位：正常入队
   - `steal != real`（有人正在窃取）：直接把任务丢到全局注入队列
   - 队列已满且无窃取：执行 `push_overflow` 溢出迁移

## 4. 溢出处理 `push_overflow`（重点）

队列满时的策略：

把当前队列后半批任务 + 新任务，整体迁移到全局注入队列

执行步骤：

1. 断言队列已满
2. CAS 修改 `head`：把 `(head,head)` → `(tail,tail)`，临时标记队列被独占，禁止窃取
3. `tail` 主动偏移半队列长度，截断本地队列
4. 构造迭代器，读取后半区所有任务
5. 连同当前新任务一起批量推送到 `overflow` 全局队列
6. 统计溢出次数

设计巧思（注释重点）

- 迁移后半段而非前半段：避免任务在「本地队列 ↔ 注入队列」无限来回震荡
- 溢出后本地队列腾出一半空间，继续接收新任务

## 5. 本地出队 `pop()`

本线程从队头取任务：

1. 循环 CAS 更新复合 head
2. 拆分 `(steal, real)`，若 `real == tail` 队空，返回 `None`
3. 计算新 `real` 索引：
   - 无窃取者 (`steal==real`)：两个索引一起递增
   - 有窃取者 (`steal!=real`)：只递增 `real`，保留窃取标记
4. CAS 尝试更新 `head`，竞争失败则重试
5. 成功后按索引读取任务并返回

内存顺序：`AcqRel`/`Acquire` 保证读写可见性。

## 6. `Drop` 析构

```rust
impl<T> Drop for Local<T> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.pop().is_none(), "queue not empty");
        }
    }
}
```

队列销毁前必须清空所有任务，防止任务泄漏；panic 场景跳过检查。

---

# 六、`Steal<T>` 窃取者侧 API（跨线程任务窃取）

`Steal` 是多消费者入口，其他 Worker 线程调用，实现 Work-Stealing 核心逻辑：

窃取规则：只偷对方队列一半任务，均衡负载。

## 1. 状态查询 `len()` / `is_empty()`

多线程安全读取队列长度，用于判断是否可窃取。

## 2. 对外窃取入口 `steal_into`

把当前队列一半任务偷到目标 `dst`（另一个 Worker 的本地队列）：

1. 先检查目标队列 `dst` 剩余空间，不够则直接放弃窃取
2. 调用 `steal_into2` 执行实际搬运
3. 统计窃取次数、窃取任务数
4. 规则：偷走 n 个，留 1 个在原队列（避免队列被偷空后频繁竞争）
5. 把大部分任务写入目标缓冲区，最后更新目标队列 `tail` 对外可见

## 3. 核心窃取逻辑 `steal_into2`

无锁批量窃取实现，分两大阶段：

### 阶段 1：CAS 抢占窃取权限

1. 加载复合 `head`，拆分 `(src_head_steal, src_head_real)`
2. 如果 `steal != real`：已有线程在偷，直接返回 `0`（规避多窃取者竞争）
3. 计算可窃取数量：`总元素数 / 2`（偷一半）
4. CAS 修改 `head`：只更新 `real` 头，保留 `steal` 标记
   - 效果：临时标记「本队列正在被我窃取」，其他窃取者止步
5. CAS 成功 → 获得窃取权限；失败则循环重试

### 阶段 2：批量搬运任务

1. 遍历待窃取区间，逐个 `ptr::read` 读源队列、`ptr::write` 写目标队列
2. 全程 unsafe：已通过 CAS 独占区间，无并发冲突

### 阶段 3：清除窃取标记

再次 CAS，把 `steal` 和 `real` 对齐：`pack(head, head)`
表示窃取完成，队列恢复正常状态，允许下一次窃取。

---

# 七、关键并发设计 & 核心亮点总结

## 1. 并发模型总览

- 单生产者（Local） + 多消费者（Steal）
- 队列：固定大小环形缓冲区
- 同步手段：原子变量 + CAS + 索引状态机，完全无锁
- 索引：加宽类型 + 双索引复合原子变量，抗 ABA、防撕裂

## 2. 双表头 `(steal, real)` 状态机（重中之重）

状态	含义	行为
`steal == real`	空闲，无窃取	可正常 `pop` / 可被窃取
`steal != real`	正在被窃取	禁止新窃取，原窃取者继续完成

用一个原子变量承载两个状态，是这套队列最难、最精巧的设计。

## 3. 环形队列 + 无符号环绕运算

全程使用 `wrapping_add` / `wrapping_sub`：

- 索引自然环形回卷，不用手动判断边界
- 是环形队列标准数学模型

## 4. 溢出策略（防队列阻塞）

本地队列满时：

1. 有窃取正在进行 → 新任务丢全局队列
2. 无窃取 → 迁移半队任务到全局队列
   
   保证本地队列始终有吞吐能力。

## 5. 任务窃取策略（Work-Steal 经典实践）

- 每次只偷一半任务：均衡各个 Worker 负载
- 偷完留至少一个任务：减少空队列竞争
- 窃取期间加标记：同一队列同时只允许一个窃取者，降低冲突

## 6. Unsafe 使用规范（Rust 工程典范）

所有 `unsafe` 都有严格前置安全保证：

1. `UnsafeCell`：靠索引规则保证读写不冲突
2. `ptr::read` / `ptr::write`：CAS 独占区间后使用
3. 裸指针转换：`make_fixed_size` 先断言长度一致
   `unsafe` 作用域极小、安全前提明确。

# 八、整体运行时序（结合 Tokio Worker 调度）

1. Worker 线程初始化 → 创建 `Local` + `Steal` 本地队列
2. 任务产生 → `push_back_or_overflow` 入本地队列，满则溢出到全局队列
3. 本线程调度 → pop() 从队头取任务执行
4. 本队列为空 → 遍历其他 Worker 的 `Steal` 句柄
5. 执行 `steal_into` 窃取一半任务，继续调度
6. 队列长期满 → 触发溢出，任务分流到全局注入队列

这就是 Tokio 多线程调度器 「本地优先 + 工作窃取 + 溢出分流」 完整运转链路。
