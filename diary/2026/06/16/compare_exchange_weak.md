# 一、为什么多处用 `compare_exchange_weak` 而非 strong

## 1. 核心区分：weak vs strong CAS

基础定义

- `compare_exchange_strong`：只有实际值！= 预期值才返回 Err；硬件层面若发生虚假失败（spurious failure）会自动重试，保证一次调用必然反映真实内存状态。

- `compare_exchange_weak`：允许虚假失败—— 哪怕实际值等于预期值，CPU 缓存、中断、流水线重排也可能返回 Err，需要外层循环重试。

性能差异（关键）

strong 为了屏蔽虚假失败，内部会循环重试；而我们代码外层已经手动写了 while/loop 循环，自带重试逻辑：

```rust
let res = self.inner.head.compare_exchange_weak(head, next, AcqRel, Acquire);
match res {
    Ok(_) => break real as usize & MASK,
    Err(actual) => head = actual, // 拿到最新值，下一轮重试
}
```

此时用 weak 收益：

1. 更少的硬件指令：不需要内置重试逻辑，CPU 开销更低，核心调度路径极致追求低延迟；
2. 无冗余重试：外层循环已经处理失败，weak 的虚假失败只是多走一轮循环，逻辑等价、性能更好；
3. 硬件流水线友好：weak CAS 阻塞更少，高并发窃取场景吞吐更高。

## 2. 什么时候必须用 strong？

只有无外层循环、只执行一次 CAS 的场景才需要 strong。

本代码所有 CAS 都包裹在无限 loop 中，天然适配 weak。

## 3. 补充风险说明

weak 虚假失败不会破坏业务逻辑：

失败时 Err 携带最新加载到的实际值，下一轮循环基于最新状态重新判断，不会丢失状态、不会死循环。

# 二、内存顺序：`Acquire` / `Release` / `AcqRel` / `Relaxed` 详解

先建立基础模型：原子操作分读 (`load`)、写 (`store`)、读改写 (CAS) 三类。

CAS 拥有两套内存顺序参数：

`compare_exchange_weak(expected, new, success_order, fail_order)`

- `success_order`：CAS 修改成功时生效的内存序
- `fail_order`：CAS 失败、仅读取时生效的内存序

## 1. `Release`（仅用于 `store` / CAS 成功分支）

作用

屏障：本线程在 `store` 之前的所有读写操作，全部不能重排到 `store` 之后；其他线程用 `Acquire` 读取该原子时，可以同步看到前面所有内存写入。

单向同步：生产者 → 消费者。

本代码场景举例

```rust
self.inner.tail.store(tail.wrapping_add(1), Release);
```

- `tail` 是队列入队下标，写入任务到 buffer 发生在 `store` 之前；
- `Release` 保证：任务写入内存一定先于 `tail` 下标更新对外可见；
- 窃取线程读取 `tail` 使用 `Acquire`，一定能看到完整写入的任务，不会读到未初始化的 `MaybeUninit`。

一句话：`Release` = 发布数据，对外公布 “数据已经写完”。

## 2. `Acquire`（仅用于 `load` / CAS 失败分支）

作用

屏障：本线程 `load` 之后的所有读写，不能重排到 `load` 之前；能同步捕获其他线程 `Release` 发布的所有写操作。

场景

1. 窃取线程 `self.0.tail.load(Acquire)`：
   读取到更新后的 `tail`，就一定能看到生产者通过 `Release` 写入 `buffer` 的完整任务，不会读到垃圾内存。

2. CAS 失败序统一填 `Acquire`：

   CAS 失败本质是一次原子 `load`，需要同步感知其他线程的修改。

一句话：`Acquire` = 接收数据，确认 “别人发布的数据我全部可见”。

## 3. `AcqRel`（仅用于 CAS 成功分支，读改写复合序）

等价于：成功时同时拥有 `Acquire` + `Release` 语义。

CAS 是读 + 写复合操作：

1. 读阶段：`Acquire`，同步读取其他线程的修改；
2. 写阶段：`Release`，发布本次修改给其他线程。

代码场景

`pop()`、`steal_into2` 核心 CAS 全部使用 `AcqRel` 作为 success_order：

```rust
self.inner.head.compare_exchange_weak(head, next, AcqRel, Acquire);
```

- `Acquire` 侧：读取旧 `head` 时，同步所有生产者 / 其他窃取者对队列 buffer、索引的修改；
- `Release` 侧：本次更新后的复合 `head` 索引，对其他线程立即可见，保证后续 load (Acquire) 能感知状态变化（如「正在窃取」标记）。

适用场景：一个原子变量同时被多线程读写、状态双向同步（本队列 head 既是生产者读写，也是所有窃取者读写）。

## 4. `Relaxed` 宽松序（无同步屏障）

无任何读写重排限制，仅保证原子变量自身读写原子；不会同步其他内存数据。

代码出现位置

`push_overflow` 溢出抢占 CAS：

```rust
.compare_exchange_weak(pack(head, head), pack(tail, tail), Release, Relaxed)
```

为什么这里失败序可以用 `Relaxed`？

失败场景只是读取当前 `head`，本线程之前已经通过 head.load(Acquire) 完整同步过队列状态；
就算 `Relaxed` 读到过期 `head`，只会重试一次循环，不会出现内存可见性问题；
降低内存屏障开销，溢出属于低频操作，进一步压缩性能损耗。

# 三、结合源码分场景对应内存顺序搭配

## 场景 1：生产者更新 tail（单纯 store）

```rust
self.inner.tail.store(new_tail, Release);
```

- 只用 `Release`：生产者写完任务，再对外发布下标；消费者 load (Acquire) 即可同步。

## 场景 2：单纯读取共享索引（load）

```rust
let (_, real) = unpack(self.inner.head.load(Acquire));
let src_tail = self.0.tail.load(Acquire);
```

- 统一 `Acquire`：必须看到完整的任务写入，防止未初始化内存访问。

## 场景 3：核心状态机 CAS（pop /steal_into2，读写共享 head）

```rust
compare_exchange_weak(..., AcqRel, Acquire)
```

- success: AcqRel：双向同步，既能看到别人的修改，也能把自己的修改同步给别人；
- fail: Acquire：失败时的 load 也要同步最新队列状态。

## 场景 4：溢出抢占 CAS（低频，前置已有 Acquire load）

```rust
compare_exchange_weak(..., Release, Relaxed)
```

- success: Release：只需要发布本次独占标记；
- fail: Relaxed：前置已经 load (Acquire)，过期值无害，弱化屏障提升速度。

# 四、总结极简版

1. weak 选择逻辑：
   
   只要 CAS 包裹在手动循环重试中，一律用 compare_exchange_weak，性能优于 strong；虚假失败仅多一轮循环，无逻辑副作用。

2. 内存顺序口诀：

   - 只写对外发布数据：`Release`
   - 只读同步别人的数据：`Acquire`
   - CAS 多线程共享状态、读写双向同步：成功序 `AcqRel`
   - 无同步依赖、仅临时读取原子值：`Relaxed`
