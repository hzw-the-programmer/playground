# 函数签名与返回值

```rust
fn run(&self, mut core: Box<Core>) -> RunResult
```

- `&self`：这里的 `self` 是 `scheduler::Context::MultiThread` 中的 `Context` 结构，它持有当前 worker 的引用、内部可变性容器和延迟唤醒队列。

- `core: Box<Core>`：该 worker 私有的 核心数据（任务队列、统计信息、LIFO 槽等），以堆分配传入，所有权进入循环。循环过程中可能被转移、替换。

- `RunResult`：实际上是 `Result<(), ()>`（或类似形式）。但整个函数永远只返回 `Err(())`，表示调度循环正常终止。将其设计为 `Result` 仅是为了在内部使用 `?` 提前退出，避免嵌套的 `if let`/`match`。

---

# 1. 重置 LIFO 启用状态

```rust
self.reset_lifo_enabled(&mut core);
```

- 若当前 core 是从其他任务那里“偷来的”（例如通过 `block_in_place` 临时转移），可能残留之前的 LIFO 槽配置。

这里显式地将 LIFO 状态恢复为默认值，保证后续调度的公平性。

---

# 2. 开始统计“处理任务”时间

```rust
core.stats.start_processing_scheduled_tasks();
```

- 从此刻起，该 worker 进入了 正在处理调度任务 的状态。统计数据用于诊断和运行时监控。

---

# 3. 主循环 `while !core.is_shutdown`

循环持续进行，直到收到停机信号（`core.is_shutdown` 为 `true`）。

## 3.1 调试断言

```rust
self.assert_lifo_enabled_is_correct(&core);
```

- 仅在 `debug_assertions` 下生效，校验 LIFO 槽的启用状态与内部标志一致。

## 3.2 追踪（Tracing）逻辑

```rust
if core.is_traced {
    core = self.worker.handle.trace_core(core);
}
```

- 若启用追踪，调用 `trace_core` 注入额外的诊断/仪器逻辑，并可能返回修改后的 core。

## 3.3 递增 tick 计数器

```rust
core.tick();
```

- `tick` 用于实现时间切片或任务预占（`coop` 机制的一部分），防止某个任务长时间独占 worker。

## 3.4 执行维护任务

```rust
core = self.maintenance(core);
```

- 维护可能包括：检查/执行全局任务队列、清理过期的定时器、处理关闭信号等。

- 该调用可能消费或修改 `core`，因此返回新的 `core` 继续使用。

## 3.5 首先检查本地队列

```rust
if let Some(task) = core.next_task(&self.worker) {
    core = self.run_task(task, core)?;
    continue;
}
```

- `next_task`：按照一定的优先级（例如 LIFO 槽 → 本地队列）获取下一个待执行的任务。

- 若拿到任务，通过 `self.run_task(task, core)` 执行它。执行时：

  + 可能因为 `block_in_place` 等原因转移 `core`；

  + 可能因为 `coop` 被迫返回；

  + 可能执行过程中遇到停机信号并提前退出。

- 执行后返回（可能已修改的）`core`，然后 `continue` 继续下一次循环。`?` 会在 `run_task` 返回错误时将 `Err` 传递出去，直接结束整个调度循环。

## 3.6 没有本地任务：结束一次统计周期

```rust
core.stats.end_processing_scheduled_tasks();
```

- 标记该轮“处理中”状态结束，因为下面将开始寻找外部工作或进入休眠。

## 3.7 尝试从其他 worker 偷任务

```rust
if let Some(task) = core.steal_work(&self.worker) {
    core.stats.start_processing_scheduled_tasks();
    core = self.run_task(task, core)?;
}
```

- `steal_work`：随机选择其他 worker，尝试从它们的本地队列偷取任务。

- 若成功，则重新开始“处理中”统计，执行该任务，然后继续循环。

## 3.8 既无本地任务也无 steal 成功 → 等待唤醒

```rust
else {
    core = if !self.defer.is_empty() {
        self.park_yield(core)
    } else {
        self.park(core)
    };
    core.stats.start_processing_scheduled_tasks();
}
```

- `self.defer` 是一个延迟唤醒队列：当 `block_in_place` 临时交出 `core` 时，某些唤醒通知可能丢失，被记录在此。若队列非空，先通过 `park_yield` 尝试快速让渡 CPU 并检查这些延迟任务，而不是立即进入深度休眠。

- 若队列为空，调用 `self.park(core)`，将当前线程挂起到等待状态（可能涉及 `tokio` 的 I/O 驱动或 `std::thread::park`），直到有其他线程唤醒它。

- 两种 park 都会消耗或重新获得 `core`，返回新的 `core`，然后重新开始统计（因为一旦被唤醒，必然有工作可做）。

# 4. 停机与清理

当 `core.is_shutdown` 变为 `true` 后，退出 `while` 循环。

## 4.1 关闭定时器（仅 unstable time 特性）

```rust
#[cfg(all(tokio_unstable, feature = "time"))]
{
    match self.worker.handle.timer_flavor { ... }
}
```

- 对于 `Alternative` 风格的定时器，需要显式关闭本地时间轮，并可能合并远程定时器以做最终清理。

## 4.2 执行停机前清理

```rust
core.pre_shutdown(&self.worker);
```

- 可能包含：执行 `defer` 中所有剩余唤醒、处理所有尚未完成的一次性任务等。

## 4.3 向运行时归还并标记 core 已关闭

```rust
self.worker.handle.shutdown_core(core);
```

- 将 `core` 所有权交还给运行时，标记该 worker 已经终止，方便后续全局清理或统计。

## 4.4 返回错误

```rust
Err(())
```

- 如上所述，返回 `Err` 表示“调度循环已结束”，与成功路径区分开来，是 Tokio 内部约定。

---

# 总体流程总结

```text
获取 core 所有权
 ↓
重置 LIFO 状态
 ↓
开始统计“处理任务”
 ↓
循环：
 ├─ 断言、追踪、tick
 ├─ 维护
 ├─ 取本地任务 → 有则执行并继续
 ├─ 无本地任务 → 结束本轮统计
 │   └─ 尝试 steal 其他 worker
 │       ├─ 有任务 → 执行并继续
 │       └─ 无任务 → park（或 park_yield）
 │            ↓ 被唤醒后重新开始统计
 ↓
（收到停机信号）
清理定时器（可选）
 ↓
pre_shutdown
 ↓
shutdown_core
 ↓
返回 Err(())
```

这个循环是 Tokio 多线程调度器 每根工作线程的搏动心跳，它精确地平衡了任务执行、工作窃取、休眠唤醒以及优雅停机，确保了高吞吐、低延迟的任务调度。
