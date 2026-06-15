# 函数签名

```rust
fn park(&self, mut core: Box<Core>) -> Box<Core>
```

- `self`：`Context` 结构，内含 `worker` 引用、`defer` 唤醒队列等。

- `core`：该 worker 的所有权核心数据（任务队列、统计、状态等），被传入并最终返回（可能已经历内部状态变更）。

返回：唤醒后（或状态变更后）的 `core`，供主循环继续使用。

---

# 1. 执行 `before_park` 回调

```rust
if let Some(f) = &self.worker.handle.shared.config.before_park {
    f();
}
```

- `before_park` 是运行时配置中的可选钩子，允许使用者在 worker 即将进入休眠前 插入自定义逻辑（例如记录指标、释放资源等）。

- 这是一个同步回调，会在真正开始 park 前调用。

---

# 2. 原子状态转换：`transition_to_parked`

```rust
if core.transition_to_parked(&self.worker) {
    // ... park 循环 ...
}
```

- `transition_to_parked` 尝试将 `Core` 的内部状态从 “已唤醒” （或中间状态）转换为 “即将休眠” 的状态。

- 它使用 原子操作或精细的状态机，确保与唤醒路径（其他线程的 `unpark`）互斥。

- 返回值：

  + `true`：状态转换成功，表示目前确实没有新工作，可以安全进入休眠。

  + `false`：转换失败，原因可能是其他线程刚刚唤醒了该 worker，或 core 已处于停机/追踪状态。此时 跳过整个 park 循环，直接到 `after_unpark` 回调，然后返回 `core`，使主循环立即重新检查任务。

这一机制避免了 唤醒丢失：若在进入休眠前的瞬间有任务被推送到本地队列且触发了 `unpark`，状态转换会失败，worker 将不会沉睡。

---

# 3. park 主循环

```rust
while !core.is_shutdown && !core.is_traced {
    // ...
}
```

循环在两种条件下退出：

- `core.is_shutdown`：运行时正在停机，worker 需要结束。

- `core.is_traced`：启用了追踪模式（如 tokio-console）。由于追踪需要持续输出事件，worker 不应真正阻塞，而应不断轮询。因此在被 trace 时，循环会立即终止，允许主循环快速重入 `run` 路径。

## 3.1 记录即将休眠的统计信息

```rust
core.stats.about_to_park();
core.stats.submit(&self.worker.handle.shared.worker_metrics[self.worker.index]);
```

- `about_to_park`：更新内部统计，记录“即将休眠”时间点。

- `submit`：将本 worker 的累计指标（如任务执行次数、poll 次数等）刷新到运行时共享的度量结构中，以便监控工具收集。

## 3.2 调用内部 park 实现

```rust
core = self.park_internal(core, None);
```

- `park_internal` 是实际阻塞线程的函数。第二个参数 `None` 表示没有超时限制。

- 实现细节：

  + 在大多数平台上，它会调用 I/O 驱动的 `park`（如 `epoll_wait` 或 `io_uring` 的等待），这样既能等待任务唤醒，也能等待 I/O 事件。

  + 如果没有 I/O 驱动，可能降级为 `std::thread::park`。

- 该调用会阻塞当前线程，直到被其他线程通过 `unpark` 唤醒、或者发生虚假唤醒（spurious wakeup）或超时。

- 返回的 `core` 可能被 `park_internal` 内部修改（例如，在唤醒前可能处理了一些内务）。

## 3.3 唤醒后记录

```rust
core.stats.unparked();
```

- 标记线程已被唤醒，用于统计休眠时长等。

## 3.4 立即执行定期维护

```rust
core.maintenance(&self.worker);
```

- 唤醒后优先执行维护任务，例如：

  + 检查全局注入队列是否有任务；

  + 清理本地队列中过期的定时器项；

  + 处理其他 worker 的释放请求等。

- 维护逻辑集成在 `Core::maintenance` 中，保证 worker 在开始下一轮任务搜索前处于“干净”状态。

## 3.5 状态转换：尝试离开 parked 状态

```rust
if core.transition_from_parked(&self.worker) {
    break;
}
```

- 与 `transition_to_parked` 对应，`transition_from_parked` 尝试将状态从 “parked” 转换回 “running/awake”。

- 返回值：

  + `true`：转换成功，表示明确有工作来源（如 `unpark` 被调用），可以退出 park 循环，重新进入主调度循环。

  + `false`：转换失败，原因可能是 虚假唤醒，或者刚刚进入 parked 状态时立即收到 wakeup 但被其他路径处理了。此时循环继续，再次进入 `park_internal`，避免空转耗费 CPU。

通过这个 双状态转换（to_parked + from_parked） 的设计，Tokio 实现了一个健壮的休眠-唤醒状态机，安全地处理了虚假唤醒和并发通知。

---

# 4. 执行 `after_unpark` 回调

```rust
if let Some(f) = &self.worker.handle.shared.config.after_unpark {
    f();
}
```

- 无论是否真正进行了 park（包括 `transition_to_parked` 失败直接跳过循环的情况），都会执行此回调。

- 用户可在其中执行自定义逻辑，如记录唤醒时间、调整调度策略等。

---

# 5. 返回 core

```rust
core
```

- 最终，返回经过可能多次休眠/唤醒循环后的 `core`。主循环会使用这个 `core` 重新开始任务搜索。

---

# 总体流程与设计思想

```text
before_park 钩子
  ↓
transition_to_parked 成功？
  ├─ 否 → 直接跳到 after_unpark → 返回 core
  └─ 是 → 进入 park 循环：
        while !shutdown && !traced:
            about_to_park / submit 指标
            park_internal (阻塞等待)
            unparked 统计
            maintenance 维护
            transition_from_parked 成功？
              ├─ 否 → 虚假唤醒，继续循环
              └─ 是 → break 循环
  ↓
after_unpark 钩子
  ↓
返回 core
```

核心作用：

- 避免丢失唤醒：在进入阻塞前进行原子状态检查，若状态变更失败则放弃休眠。

- 处理虚假唤醒：通过 `transition_from_parked` 判断是否是真实的唤醒，虚假唤醒时重新进入阻塞。

- 追踪模式兼容：当 `core.is_traced` 为 true 时，完全跳过阻塞循环，保持 worker 持续轮询。

- 优雅停机：一旦 `core.is_shutdown` 被设置，立即退出阻塞循环，让主循环结束 worker 运行。

- 统计与维护：每次休眠前后记录度量数据，唤醒后立即执行维护，保证调度器健康度。

`park` 是 Tokio 工作线程 节能与响应性平衡的关键，通过与 I/O 驱动集成、原子状态机、和回调扩展点，使 worker 能在无任务时高效休眠，在有任务时迅速唤醒。
