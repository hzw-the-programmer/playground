# 1. 防 panic 被静默：AbortOnPanic

```rust
#[allow(dead_code)]
struct AbortOnPanic;

impl Drop for AbortOnPanic {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!("worker thread panicking; aborting process");
            std::process::abort();
        }
    }
}

#[cfg(debug_assertions)]
let _abort_on_panic = AbortOnPanic;
```

目的：在测试环境中，worker 线程内的 panic 通常很难被外部捕获，容易导致测试挂起或错误信息不明确。因此，在 debug 断言开启（即非 release 模式）时，通过 RAII 守卫 `AbortOnPanic`，让线程发生 unwind 时直接 `abort` 整个进程。

- `std::thread::panicking()` 检查当前线程是否正在 panic（即正在执行析构函数）。

- `std::process::abort()` 立即终止进程，不进行任何清理，生成 core dump（如果系统配置允许），便于调试。

- `_abort_on_panic` 的生命周期贯穿整个 `run` 函数，当函数退出（无论正常还是 panic）时触发 `Drop`，从而执行上述检查。

注意：此逻辑仅在 `debug_assertions` 下生效，release 模式下不会编译，避免线上误杀。

---

# 2. 独占获取 worker core

```rust
let core = match worker.core.take() {
    Some(core) => core,
    None => return,
};
```

`worker.core` 是一个 `AtomicCell<Core>` 类型，被多个线程共享。这里通过 `take()` 原子性地取出 `Core` 所有权：

- 若成功取得 `Some(core)`，说明本线程获得了运行此 `worker` 的权限。

- 若得到 `None`，说明其他线程已经取走 `core` 并在运行此 `worker`，当前线程无需做任何事，直接 `return` 退出。

---

# 3. 记录 worker 度量信息

```rust
worker.handle.shared.worker_metrics[worker.index]
    .set_thread_id(thread::current().id());
```

将当前操作系统线程的 ID 记录到该 worker 的指标结构中，方便监控和调试。`worker.index` 是该 worker 在多线程池中的编号。

---

# 4. 进入 Tokio 运行时上下文

```rust
let handle = scheduler::Handle::MultiThread(worker.handle.clone());

crate::runtime::context::enter_runtime(&handle, true, |_| {
    // ... 内部逻辑
});
```

`handle` 封装了运行时句柄，用于任务在运行时内部访问调度器（如 spawn 新任务）。

`enter_runtime` 将当前线程标记为 运行时线程，设置线程局部变量（`tokio::runtime::context`）指向该运行时，确保在闭包内执行的所有代码都能正确获取到运行时资源。第二个参数 `true` 可能表示 阻塞上下文，允许某些阻塞操作使用 `block_in_place`。

---

# 5. 设置调度器上下文并运行调度循环

```rust
let cx = scheduler::Context::MultiThread(Context {
    worker,
    core: RefCell::new(None),
    defer: Defer::new(),
});

context::set_scheduler(&cx, || {
    let cx = cx.expect_multi_thread();

    assert!(cx.run(core).is_err());

    cx.defer.wake();
});
```

- 构建 `Context`：

  + `worker`: 当前 worker 的引用。

  + `core`: 用 `RefCell::new(None)` 占位，因为真正的 `core` 稍后会直接传给 `cx.run()`。这里 `None` 只是初始值，`RefCell` 用于在调度过程中允许内部可变地存放临时取出的 `Core`。

  + `defer`: `Defer` 是一个唤醒队列，用于存放由于 `block_in_place` 等原因被推迟通知的任务。

- `set_scheduler`：将当前线程的调度器上下文设为 `cx`，之后在当前线程上执行的任何任务都可以通过 `context::with_scheduler()` 获取到该调度器。

- `cx.run(core)`：

这是真正的 调度循环。它会不断从本地的任务队列（或全局队列）取出任务并执行，直到没有更多任务可运行或需要停机。
函数返回 `Result`，但 实际上永远会返回 `Err`，表示循环终止（正常运行结束或遇到停机信号）。设计为 `Result` 仅是为了在内部使用 `?` 提前退出。
这里的 `assert!` 确保 worker 循环必须是因“完成”而退出，而非意外成功返回。

- `cx.defer.wake()`：

当调度循环结束后，需要处理 延迟通知队列。
`block_in_place` 可能会临时将 worker 的 `core` 转移出去，导致某些唤醒通知丢失。此时这些任务被记录在 `defer` 中，在 worker 结束前必须主动唤醒它们，确保没有任务被永远遗忘。

---

# 整体流程总结

- Panic 守卫（仅 debug）：保证 worker 线程 panic 时进程直接 abort，防止测试被卡住。

- 获取 core：原子性地获取 worker 的运行权，若已被其他线程获取则直接退出。

- 记录线程 ID 用于监控。

- 进入运行时上下文，让当前线程获得运行时能力。

- 设置调度器上下文并启动主循环：

  + 运行 `cx.run(core)` 进入任务调度循环。

  + 循环退出后，处理 deferred 唤醒队列，避免任务遗漏。

这个函数是整个 Tokio 多线程调度器的 工作线程入口，保证了线程安全、panic 处理、任务调度和资源清理的正确性。
