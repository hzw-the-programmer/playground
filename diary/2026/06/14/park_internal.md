`park_internal` 是 Tokio 多线程调度器中 真正执行线程阻塞的内部方法。它负责安全地挂起当前 worker 线程，直到超时或收到唤醒事件（如 I/O 就绪、任务推送），并在前后完成必要的状态保护、定时器维护及通知逻辑。

---

# 函数签名

```rust
fn park_internal(&self, mut core: Box<Core>, duration: Option<Duration>) -> Box<Core>
```

- `self`：调度上下文，内含 worker 引用、`RefCell<Option<Box<Core>>>` 的 `core` 字段、`defer` 唤醒队列等。

- `core`：当前 worker 的完整核心数据（任务队列、parker、统计等）。

- `duration`：可选超时时长。`None` 表示无限期阻塞；`Some(d)` 表示最多阻塞 `d` 时间。

返回：唤醒后的 `core`（可能经过了维护和状态变更）。

---

# 1. 断言 LIFO 槽状态

```rust
self.assert_lifo_enabled_is_correct(&core);
```

- 仅在 debug 断言下生效，确保 LIFO 状态标志与实际配置一致。

---

# 2. 取出 Parker，并将 Core 暂存到上下文

```rust
let mut park = core.park.take().expect("park missing");
*self.core.borrow_mut() = Some(core);
```

为何这样做？

`Core` 包含唤醒线程所需的 `Parker`。而实际阻塞调用 (`park.park(...)`) 会释放当前线程的执行权。在阻塞期间，其他线程可能通过 `unpark` 尝试唤醒该 worker，如果此时 `Parker` 还嵌在 `Core` 内部，其他线程需要访问整个 `Core` 才能调用 `unpark`，这会引入复杂的同步和所有权问题。

因此，Tokio 采用分离模式：

1. 从 `core` 中取出 `Parker`，获得其所有权。

2. 将剩余 `core`（不含 parker）放入 `self.core` （`RefCell<Option<Box<Core>>>`）。

3. 阻塞结束后再从 `self.core` 取回 `core`，并将 `Parker` 放回。

这样的好处：

- 其他线程需要 `unpark` 时，只需操作 `Parker`，无需持有 `Core` 锁。

- `Parker` 本身通常是一个 `Arc` 共享的内部状态，可以安全跨线程使用。

- 确保 `core` 在阻塞期间保存在上下文中，其他路径（如 `block_in_place`）可能通过 `self.core` 临时借用 `core`。

---

# 3. 处理定时器相关的准备（#[cfg(feature = "time")]）

```rust
#[cfg(feature = "time")]
let (duration, auto_advance_duration) = match self.worker.handle.timer_flavor {
    TimerFlavor::Traditional => (duration, None::<Duration>),
    #[cfg(tokio_unstable)]
    TimerFlavor::Alternative => {
        let MaintainLocalTimer {
            park_duration: duration,
            auto_advance_duration,
        } = self.maintain_local_timers_before_parking(duration);
        (duration, auto_advance_duration)
    }
};
```

- `Traditional` 定时器：直接使用传入的 `duration`，不做额外处理。

- `Alternative`（实验性）定时器：调用 `maintain_local_timers_before_parking`，它会：

  + 检查本地时间轮，计算最近一次定时器触发的时间。

  + 若实际需要等待的时间比传入的 `duration` 更短，则缩减超时时间。

  + 返回更新后的 `park_duration` 和 `auto_advance_duration`（用于后续时间轮自动推进）。

设计要点：

这个调用必须在 Parker 取出之后进行，因为新定时器模型的 `schedule_local` 可能会延迟通知，如果 Parker 还在 `core` 里会引发竞态。详情见 `Handle::schedule_local` 的注释。

---

# 4. 执行实际阻塞

```rust
let had_driver = if let Some(timeout) = duration {
    park.park_timeout(&self.worker.handle.driver, timeout)
} else {
    park.park(&self.worker.handle.driver)
};
```

- 根据 `duration` 选择：

  + `park_timeout`：带超时的阻塞，最多等待 `timeout` 时间。

  + `park`：无限期阻塞。

- 两者都接收 `&self.worker.handle.driver`（即 I/O 驱动），用于在阻塞时注册 I/O 事件等待。

  - `had_driver`：返回一个布尔值，指示本次阻塞是否是因为 I/O 驱动有事件而醒来（或有其他驱动原因）。它会影响后续是否通知其他 worker 的逻辑。

# 5. 唤醒延迟队列

```rust
self.defer.wake();
```

- `Defer` 队列保存了那些在 `block_in_place` 执行过程中被推迟的任务唤醒通知。

- 线程刚被唤醒，是触发这些延迟通知的最佳时机，以确保相关任务能被后续循环及时执行。

---

# 6. 定时器唤醒后维护（#[cfg(feature = "time")]）

```rust
#[cfg(feature = "time")]
match self.worker.handle.timer_flavor {
    TimerFlavor::Traditional => { let _ = auto_advance_duration; }
    #[cfg(tokio_unstable)]
    TimerFlavor::Alternative => {
        self.maintain_local_timers_after_parking(auto_advance_duration);
    }
}
```

- `Alternative` 定时器：调用 `maintain_local_timers_after_parking`，用之前准备的 `auto_advance_duration` 推进本地时间轮，处理超时任务。

- `Traditional` 定时器：仅消除未使用变量警告。

顺序关键：此维护必须在 Parker 放回 `core` 之前完成，原因同前——避免 `schedule_local` 延迟通知与 parker 内部状态的竞态。

---

# 7. 重新组装 Core

```rust
core = self.core.borrow_mut().take().expect("core missing");
core.park = Some(park);
core.had_driver = had_driver;
```

- 从 `self.core` 取回之前暂存的 `Core`。

- 将 `Parker` 放回 `core.park`。

- 记录 `had_driver`，该标志会影响 `core.should_notify_others()` 的判断。

---

# 8. 通知其他 Worker（如需要）

```rust
if core.should_notify_others() {
    self.worker.handle.notify_parked_local();
}
```

- `should_notify_others` 检查是否存在某种情况（例如：

  + 全局队列中有新任务；

  + 本地队列中有任务但未能及时调度；

  + I/O 事件就绪等），使得有必要唤醒另一个可能休眠的 worker 来协助处理。

- `notify_parked_local()` 会尝试唤醒一个处于 parked 状态的 worker（如果存在）。

---

# 9. 返回重构后的 Core

```rust
core
```

- 返回包含更新后的 `parker`、`had_driver` 标志、以及可能已经过内部维护的 `core`，供外层调度循环继续使用。

---

# 核心设计思想

- 所有权分离：通过临时转移 `Parker` 所有权，保证了阻塞期间的其他线程唤醒不会触及 `Core` 锁，从而避免了复杂并发控制。

- 精确的超时控制：结合定时器模型，动态调整超时，确保在定时器到期时能及时唤醒。

- 延迟唤醒处理：`defer.wake()` 确保那些因 `block_in_place` 而错失的通知在唤醒后立即被处理。

- 协作通知：根据唤醒原因，决定是否通知其他闲置 worker，实现负载均衡。

- 追踪与调试：多处断言和度量统计，确保状态机正确。

`park_internal` 是 Tokio 调度器 高效休眠机制 的精髓，它通过细致的状态分离与协调，使工作线程能在无任务时安静等待，而在有事件时迅速响应，同时保持了代码的健壮与可扩展性。
