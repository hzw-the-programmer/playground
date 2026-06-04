# 1. 什么是 cancel-safe（取消安全性）？

Cancel-safe 指：当一个 `future` 被取消（`drop`）时，它代表的异步操作不会留下不一致的中间状态，导致不可恢复的数据丢失或资源泄漏。并且，如果之后重新发起同样的操作，能够正确继续或重试，不会依赖被丢弃的 `future` 的中间进度。

简单说：你的 `future` 可以安全地被“杀死”，系统状态仍然完好。

反之，如果一个 `future` 被取消后会破坏状态、丢失数据或导致未来操作无法正常进行，它就是 非 cancel-safe 的。

# 2. 为什么需要关心 cancel-safe？

在同步 Rust 中，函数要么执行完，要么 `panic`，不存在“执行到一半就被丢弃”。但在异步 Rust 中，`future` 是状态机，它可能在某次 `.await` 之后被恢复，也可能在 `.await` 之前就被丢弃。

以下场景会隐式地取消 `future`：

- `select!` 选中一个分支后，立即 `drop` 所有其他分支的 `future`。

- `tokio::time::timeout` 超时后，`drop` 被超时的 `future`。

- `JoinHandle::abort()` 取消一个后台任务（执行过程中被 `drop`）。

- 持有 `future` 的变量离开作用域或被显式 `drop`。

如果你的 `future` 在被 `drop` 时已经修改了某些外部状态（比如往缓冲区写了部分数据、增加了一个计数器、发送了半条消息），那么这些“半吊子”操作就会永久影响系统，而调用者对此毫不知情。

典型苦果：

你用 `select!` 给 `read_exact(&mut buf)` 加了超时，超时后 `read_exact` 被 `cancel`。但这时 `buf` 里可能已经填充了 50 字节，然而 `future` 没了，你不知道到底读了多少，这 50 字节也丢失了，下次再读可能会造成协议错乱。

# 3. Cancel-safe 与 Cancel-safe 的标准

一个 `future` 是 cancel-safe 的，通常满足以下条件之一：

- 无副作用的纯等待：如计时器、信号、条件变量通知。取消它们仅意味着“不再关心这个事件”，不影响全局。

- 原子性操作：操作要么整体完成，要么不产生任何外部可见的变化，且在未完成时被丢弃不会留下痕迹。例如 `mpsc::Receiver::recv()` —— 当它被取消时，消息仍在通道中，下一次 `recv()` 依然能收到它（通道消息不会被“取走一半”）。

- 显式提供进度信息：如 `AsyncReadExt::read()`，它返回实际读取的字节数，并且不保证填满缓冲区。每次调用是一个独立的尝试，被取消仅表示这次尝试失败，没有隐式状态残留。

非 cancel-safe 的典型代表：

- `AsyncReadExt::read_exact()` —— 会多次调用 `read` 直到填满缓冲区，内部记录了已读位置，一旦被取消，缓冲区和内部计数器的状态丢失，已读数据也丢失。

- `tokio::io::copy_buf()` 等需要多次 I/O 操作的组合器。

- 某些多层状态机：比如在握手期间发送了部分协议帧后等待回复，被取消可能让对端永远挂起。

- 任何你在 `async fn` 或 `async` 块中手动维护的、跨过 `.await` 的可变状态（不是委托给 cancel-safe 原语的），一旦在 `.await` 处被取消，状态可能不一致。

# 4. Cancel-safe 检查清单

判断一个 `future` 是否 cancel-safe，问自己三个问题：

- 被取消时，它是否已经修改了外部状态？
  
  如果它只是在等待某个事件（如信道、计时器），没有修改外部，那通常是安全的。

- 如果未来重新创建同样的 `future`，逻辑能正确继续吗？

  例如 `rx.recv()` 被取消，下次再 `recv()` 仍能拿到那条消息，说明它是幂等的安全取消。而对于 `read_exact`，下次再 `read_exact` 你不知道应该从哪开始。

- `future` 的 `drop` 实现是否做了必要的清理？

  有些 `future` 会在 `drop` 时主动重置状态（比如释放锁、回滚操作），也可以视为 cancel-safe，但 Rust 的 `future` 很少这样做，因为成本高且运行时不可控。通常靠设计避免。

# 5. 典型安全的 Future 举例

Future	安全性	原因
`tokio::time::sleep()`	✅ 安全	纯等待，被取消无非是闹钟白定了。
`tokio::sync::mpsc::Receiver::recv()`	✅ 安全	消息保留在通道中，下次还能取。
`tokio::sync::oneshot::Receiver`	⚠️ 有条件安全	接收端取消安全，但发送端如果依赖发送成功，可能被永久阻塞。
`tokio::signal::ctrl_c()`	✅ 安全	等待 OS 信号，取消只是不再等待。
`TcpStream::readable()`	✅ 安全	仅等待可读事件，不涉及数据消费。
`AsyncReadExt::read()`	✅ 安全	返回实际读取字节数，无内部缓冲进度。
`AsyncWriteExt::write()`	✅ 安全	返回实际写入字节数，无内部缓冲进度。
`AsyncWriteExt::write_all()`	❌ 不安全	内部循环写，部分写后丢弃进度。
`AsyncReadExt::read_exact()`	❌ 不安全	内部循环读，已读部分将丢失。
手动 `loop { ... .await }` 维护状态	❌ 多半不安全	除非你专门设计成 cancel-safe。

# 6. 如何变“不安全”为“安全”？

你有几个武器：

## a) 使用原子单次操作代替复合操作

用 `read()` + 循环代替 `read_exact()`，并在每次循环中自己记录进度。因为你的状态记录在循环变量里，而不是隐藏在 `future` 内部，即使循环被 `select!` 取消，你仍持有 `buf` 和 `n` 等进度变量，可以安全恢复。

```rust
// 安全的精确读取（cancel-safe）
async fn read_exact_cancel_safe(
    mut reader: impl AsyncReadExt + Unpin,
    buf: &mut [u8],
) -> io::Result<()> {
    let mut n = 0;
    while n < buf.len() {
        // 单次 read 是 cancel-safe
        let cnt = reader.read(&mut buf[n..]).await?;
        if cnt == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "eof"));
        }
        n += cnt;
    }
    Ok(())
}
```

这样即使外部 `select!` 取消了这个循环 `future`，调用者仍然持有 `buf` 和 `n`，可以从上次进度继续。

## b) 将非安全操作 `spawn` 到独立任务

使用 `tokio::spawn` 把耗时且非安全的 `future` 抛给另一个任务，然后用 `JoinHandle` 或 `oneshot` 来接收结果。`select!` 只会取消接收器，真正的任务仍在后台执行，直到完成。

```rust
let (tx, rx) = oneshot::channel();
tokio::spawn(async move {
    // 这个 read_exact 不会被取消，因为它在另一个任务中
    let mut buf = vec![0u8; 1024];
    reader.read_exact(&mut buf).await?;
    let _ = tx.send(buf);
});

select! {
    result = rx => { /* 收到完整数据 */ }
    _ = sleep(Duration::from_secs(5)) => { /* 超时 */ }
}
```

代价：如果超时，后台任务仍在读取，可能浪费资源。你需要根据业务决定是丢弃还是等待。

## c) 使用取消安全版本的库

Tokio 提供了一些 cancel-safe 的替代方法，比如：

- `tokio::io::AsyncReadExt::read()` 替代 `read_exact()`

- `tokio_util::io::StreamReader` 帮助构建 cancel-safe 流处理

- 对于 TCP 流，`AsyncReadExt::read()` 安全，`AsyncWriteExt::write()` 安全。

## d) 利用 `Drop` 主动回滚

极少使用，但你可以自定义 `future`，在其 `Drop` 中修复被破坏的不变式。但实现复杂且容易出错，通常不推荐。

# 7. 设计自己的 Cancel-safe API

如果你正在写库或公共组件，应明确文档说明你的 `future` 是否 cancel-safe。设计指南：

- 对于“询问-响应”模型：尽量使操作无副作用，或提供“查询进度”的接口，而不是隐藏状态。

- 避免在 `future` 的中间状态修改外部变量，所有状态变更尽量集中在最终结果返回前进行。

- 如果必须修改外部状态，考虑使用事务语义：准备所有数据，最后原子提交；被取消时丢弃准备区。

- 暴露进度：比如返回 (已处理数量, 结果)，让调用者可以在被取消后重建状态。

经典例子：Tokio 的 `mpsc::Receiver` 和 `oneshot` 都设计为 cancel-safe，因为它们的“接收”不移动数据，只有成功拿到数据才从通道移除。

# 8. 常见误解与陷阱

## 误解一：`async` 块里所有 `.await` 都是安全的

危险！以下代码不安全：

```rust
async fn risky(mut stream: TcpStream, mut buf: Vec<u8>) -> io::Result<Vec<u8>> {
    stream.read_exact(&mut buf).await?;  // ❌ 非 cancel-safe
    Ok(buf)
}
```

如果在 `read_exact` 的 `.await` 点被取消，`buf` 内容部分更新，但函数返回 `Ok(buf)` 这个路径再也不会被执行，数据丢失。

## 误解二：`select!` 内使用 `biased` 可以避免 `cancel`

不能。`biased` 只改变就绪选择的优先级，不改变“其他分支被丢弃”的事实。被跳过的分支依然被 `cancel`。

## 误解三：`tokio::spawn` 的任务永远不会被 `cancel`

会。如果你将 `JoinHandle` 丢弃（不 `.await`），`spawn` 出的任务并不会自动取消，它会继续执行（这被称为“detach”）。但如果调用了 `abort()` 或运行时终止，任务会被取消，其中 `future` 的 `drop` 会运行。所以即便是 `spawned` 任务，如果可能被 `abort`，其内部也要考虑 cancel-safe。

# 9. 总结：Cancellation is always possible

在异步 Rust 中，任何 `future` 都可能在任何 `.await` 处被取消。这不是 bug，而是设计 —— 它允许构建超时、竞速、优雅关闭等强大模式。但你必须为此负责，确保你使用的每个 `future` 要么天生 cancel-safe，要么你封装它成为 cancel-safe。

记住黄金法则：

- 用 `read()` 而不是 `read_exact()`，除非你做了额外保护。

- 别在 `select!` 里直接放非安全 `future`，要么重构成安全版本，要么 `spawn` 出去。

- 写异步代码时，时刻问自己：“如果这个 `.await` 被取消，我的状态还一致吗？”

当你养成了这个思维习惯，就真正掌握了 cancel-safe，能写出经得起组合器折腾的健壮异步代码。
