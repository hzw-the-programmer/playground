# Tokio Worker next_task 逐段完整解析

这段是 Worker 线程核心调度循环取任务逻辑，决定 Worker 去哪里拿任务，完整实现 Tokio 多线程调度优先级：

本地队列优先 → 周期性查全局注入队列 → 本地空则批量拉取全局任务缓存到本地

配合你前面看懂的 Local/Steal 本地队列、溢出分流逻辑，形成完整调度闭环。

---

# 一、函数整体作用

```rust
fn next_task(&mut self, worker: &Worker) -> Option<Notified>
```

每次调度循环调用，返回一个待执行任务；返回 None 代表当前无任务，Worker 进入休眠 / 窃取其他 Worker 队列。

调度优先级规则：

1. 大部分 tick：先取本地队列任务
2. 每间隔 global_queue_interval 个 tick：先查全局注入队列，再本地（均衡全局任务）
3. 本地空、全局有任务：批量拉一批全局任务塞进本地队列，留一个立刻执行

---

# 二、第一段：分支判断 self.tick % self.global_queue_interval == 0

逻辑：周期性切换取任务顺序，均衡全局负载

```rust
if self.tick % self.global_queue_interval == 0 {
    // 动态调整拉取全局队列的间隔
    self.tune_global_queue_interval(worker);

    // 先全局注入队列，再本地队列
    worker.handle.next_remote_task()
        .or_else(|| self.next_local_task())
} else {
    // 常规路径：先本地，后全局
    // ...下方一大段常规逻辑
}
```

关键细节

1. `tick`：Worker 每取一次任务计数 + 1；
2. `global_queue_interval`：动态可调阈值，`tune_global_queue_interval` 会根据全局任务负载自动调大 / 调小；
   - 全局任务多 → 缩小间隔，频繁去拉全局；
   - 全局任务少 → 放大间隔，尽量只走本地队列，减少全局锁竞争。
3. `next_remote_task()` = 从全局注入队列 (inject queue) 拿任务；
4. 周期分支顺序：全局优先，避免全局任务堆积、个别 Worker 过载。

---

# 三、else 常规主路径（99% 调度走这里：本地优先）

```rust
let maybe_task = self.next_local_task();
if maybe_task.is_some() {
    return maybe_task;
}
```

1. 先调用 `next_local_task()` 从当前 Worker 专属本地 run_queue 取任务；
2. 本地有任务直接返回，不走后续全局拉取逻辑；
3. 只有本地彻底空了，才进入「从全局批量拉任务」分支。

下一步判断全局队列是否为空

```rust
if worker.inject().is_empty() {
    return None;
}
```

全局注入队列也没任务 → 直接返回 `None`，Worker 会进入任务窃取阶段（偷其他 Worker 的本地队列）。

---

# 四、计算可拉取并缓存到本地的任务上限 `cap`

```rust
let cap = usize::min(
    // 1. 本地队列剩余空位
    self.run_queue.remaining_slots(),
    // 2. 最多只存半队列容量
    self.run_queue.max_capacity() / 2,
);
```

两个约束取最小值，双重限制批量拉取数量：

约束 1：remaining_slots() 本地剩余空位

本地环形队列有固定容量 256，不能塞爆；
注释说明：其他线程只能偷任务、删除任务，不会写入本地队列，因此剩余空位是稳定可信值。
即使有其他 Worker 正在窃取，剩余空位只会变多、不会变少，push 一定安全不溢出。

约束 2：max_capacity() / 2 最多半队

和之前 push_overflow 溢出逻辑强关联：
本地队列后半区任务溢出时会被迁移到全局注入队列；
我们从全局拉来的任务只存本地前半区，保证这批新任务永远不会因为队列满被再次分流回全局，避免任务在「本地 ↔ 全局」无限来回震荡。

`cap` 含义

本次最多从全局拉 `cap` 个任务缓存进本地队列。

---

# 五、计算实际拉取数量 `n`

```rust
let n = usize::min(
    // 全局任务均分：全局总数 / Worker线程数 +1
    worker.inject().len() / worker.handle.shared.remotes.len() + 1,
    cap,
);
// 最少拉1个
let n = usize::max(1, n);
```

均分逻辑核心设计

假设线程池有 `N` 个 Worker，全局队列有 `M` 个任务：
每个 Worker 最多只拿 `M/N + 1` 个，目的是全局任务均衡分发，不会单个 Worker 一次性把全局任务全部抢光，留给其他线程无任务可执行。
再和前面 `cap` 取小，不能超过本地队列容纳上限；
最后保底至少拉 1 个，保证一定有任务可以立刻执行。

---

# 六、加锁批量弹出全局任务

```rust
let mut synced = worker.handle.shared.synced.lock();
// safety: 传入合法的 inject 同步句柄
let mut tasks = unsafe { worker.inject().pop_n(&mut synced.inject, n) };
```

1. 全局注入队列是多线程共享，必须加互斥锁 `synced.lock()`；
2. `pop_n(n)`：一次性从全局队列弹出最多 n 个任务，返回迭代器；
3. `unsafe` 仅约束内部队列生命周期，外部调用保证参数合法，无内存风险。

---

# 七、分离任务：一个立即执行，其余缓存本地

```rust
// 第一个任务直接返回，马上执行
let ret = tasks.next();

// 剩下全部推入本地 run_queue 缓存
self.run_queue.push_back(tasks);

ret
```

设计意图

1. 第一个任务 `ret` 直接返回给调度循环，不用进本地队列再 `pop`，省去一次本地队列操作，减少开销；
2. 剩余任务批量 `push_back` 存入本地环形队列，后续调度循环直接走本地快速路径，不用频繁竞争全局锁；
3. 完美契合前面本地队列「只存前半区」约束，不会触发溢出回流。

---

# 八、整条调度链路完整流程梳理

1. Worker 循环调用 `next_task` 获取待执行任务
2. 判断 tick 周期：
   - 周期到达：先查全局注入队列，再本地；
   - 普通 tick：优先本地队列 `next_local_task`
3. 本地有任务 → 直接返回本地任务
4. 本地空，检查全局注入队列：
   - 全局也空 → 返回 None，进入跨 Worker 任务窃取逻辑；
   - 全局有任务：
     + 计算本地可容纳上限 cap（空位 / 半队容量取小）
     + 均分全局任务，算出本次拉取 n 个（最少 1）
     + 全局队列加锁，批量弹出 n 个任务
     + 拆分：1 个立刻执行，剩余批量存入本地队列
     + 返回第一个任务

---

# 九、关键设计权衡点（Tokio 核心调度优化）

1. 本地优先，减少全局锁竞争
   全局注入队列带锁，频繁争抢性能差；绝大多数任务存在 Worker 本地无锁队列，只有本地空才碰全局锁。
2. 周期性切换查询顺序，防全局饥饿
   长期本地任务多的 Worker 会忽略全局，定时切换「全局优先」保证全局任务不会永久堆积。
3. 批量拉取、缓存本地
   一次加锁拿一批任务，多次调度复用，大幅降低全局锁争抢频率。
4. 半区限制防止任务震荡回流
   从全局拉的任务只放本地前半区，永远不会在队列满时被 push_overflow 迁回全局，减少重复搬运开销。
5. 全局任务均分策略
   避免单个 Worker 垄断全局所有任务，均衡多线程负载。
6. 至少返回 1 个任务
   保证只要全局有任务，当前 Worker 一定能拿到可执行任务，不会白加锁。
