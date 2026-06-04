# 一、基本作用

`tokio::select!` 同时轮询多个 `future`，当任意一个 `future` 完成时，执行对应的分支代码块，并丢弃（取消）其余所有未完成的 `future`。整条 `select!` 表达式的值就是被选中分支的返回值（各分支返回类型必须相同）。

典型场景：

- 同时接收多个通道的消息

- 为一个操作加上超时

- 监听退出信号的同时执行业务逻辑

# 二、基础语法

```rust
use tokio::select;

async fn some_async_operation() -> String {
    "hello".to_string()
}

async fn example() {
    let mut channel = tokio::sync::mpsc::channel(10);
    let (tx, mut rx) = channel;

    let result = select! {
        // 模式 = future 表达式 => 处理代码块
        value = rx.recv() => {
            println!("收到消息: {:?}", value);
            value
        }
        s = some_async_operation() => {
            println!("异步操作完成: {}", s);
            Some(s)
        }
    };
    // result 的类型是 Option<String>，因为两个分支都返回 Option<String>
}
```

宏语法规则：

- 每个分支 `pattern = <future> => handler`

- `<future>` 必须实现 `Future`

- `pattern` 用于匹配 `future` 的 `Output`，可以带 `if` 守卫吗？不可以，只支持普通的模式匹配

- `handler` 是同步代码块（不能直接 `.await`，若要异步，可在块内启动新任务）

- 可以有一个 `else => handler` 分支，当所有 `future` 都已完成（即返回 `Poll::Ready(None)` 或类似“关闭”状态）且无其他分支就绪时执行

# 三、核心特性详解

## 1. 随机公平性（默认）

默认情况下，`select!` 随机选择一个就绪的分支执行，以避免偏向导致饥饿。可通过 `biased;` 关键字改为按声明顺序优先级：

```rust
select! {
    biased;   // 现在会优先检查第一个分支，再第二个，依此类推
    _ = tokio::time::sleep(Duration::from_secs(1)) => { println!("1秒"); }
    _ = tokio::time::sleep(Duration::from_millis(100)) => { println!("100毫秒"); }
}
// 因为 biased，即使 100 毫秒先到，如果 1 秒也同时就绪，仍然先打印 "1秒"
```

## 2. 模式匹配

分支左侧可以使用任何合法的 Rust 模式，解构 future 的输出：

```rust
select! {
    Some(msg) = rx.recv() => { /* 处理消息 */ }
    Ok(data) = async_read() => { /* 处理读到的数据 */ }
    _ = tokio::signal::ctrl_c() => { /* 收到 Ctrl+C */ }
}
```

如果模式匹配失败怎么办？该分支会被视为从未就绪，`select!` 将继续等待其他分支。这很有用，但也要小心：如果所有分支的模式都永远无法匹配，`select!` 就会永远阻塞。

## 3. `else` 分支

当所有指定的异步操作都返回表示“流结束”的值（例如 `recv()` 返回 `None`）时，才执行 `else` 块：

```rust
select! {
    msg = rx.recv() => { /* 处理消息 */ }
    else => { println!("通道已关闭"); break; }
}
```

注意：`else` 只在所有 `future` 同时完成且都是终止状态时才触发，不是“其他任何情况”。

# 四、取消安全性（极其重要）

`select!` 会在一个分支完成后，立即 `drop` 其余 `future`。如果被 `drop` 的 `future` 已经修改了某些状态但未完全完成，可能会导致数据丢失或逻辑错误。这就是“cancel safety”（取消安全性）问题。

cancel-safe（安全）的 `future`：被取消后，状态不会遭破坏，再次使用相同的 `future` 或重新创建同样的操作能正确继续。例子：

- `tokio::sync::mpsc::Receiver::recv()`

- `tokio::time::sleep()`

- `tokio::signal::ctrl_c()`

- `tokio::net::TcpStream::readable()`

非 cancel-safe 的 `future`：例如：

`tokio::io::AsyncReadExt::read_exact(&mut buf)` ：如果读了部分字节就被取消，`buf` 中已有部分数据，但 `future` 消失，这些字节就丢失了，而且无法知道读了多少。

`tokio::sync::oneshot::Receiver` ：虽然它本身可安全取消，但如果发送端依赖接收成功来改变状态，取消会造成发送端永远挂起。

如何安全使用非 cancel-safe future？

把它变成 cancel-safe：

- 使用 `tokio::io::AsyncReadExt::read()` 替代 `read_exact()`，自行循环处理。

- 将耗时的非安全 `future` 单独 `spawn` 到另一个任务，通过 `JoinHandle` 或 `oneshot` 管道与 `select!` 交互。因为取消的只是接收端，真实操作仍在后台运行。

- 使用 `tokio_util::io::StreamReader` 或 `tokio::sync::Mutex` 配合手动轮询状态机。

示例：为 I/O 读取提供超时（采用安全的 `read` + 循环）

```rust
async fn read_with_timeout(stream: &mut TcpStream, buf: &mut [u8]) -> io::Result<usize> {
    select! {
        res = stream.read(buf) => res,
        _ = tokio::time::sleep(Duration::from_secs(5)) => {
            Err(io::Error::new(io::ErrorKind::TimedOut, "读取超时"))
        }
    }
}
```

这里的 `stream.read(buf)` 是 cancel-safe 的，因为它直接返回已读取的字节数，即便被取消，也仅仅是一次单次读取尝试，`buf` 内容未定义，但不会造成“部分进度丢失无法恢复”的问题。但注意，如果你需要精确读取 N 字节，就要自己维护状态。

# 五、实现原理简析

`select!` 展开后大致会生成以下结构（伪代码）：

- 为每个分支的 `future` 创建变量（`Pin<&mut Future>`）

- 构造一个匿名结构体或通过 `poll_fn` 循环轮询这些 `future`

- 用 `unsafe` 同时持有多个 `Pin<&mut>` ，因为 Rust 不允许同一个作用域有多个可变引用，但这里靠宏保证没有重叠且不会移动

- 默认随机打乱分支顺序；若声明 `biased` 则按书写顺序

- 任何一个 `future` 返回 `Ready`，提取值并运行对应 `handler`，然后 `drop` 其他 `future`

- 如果所有 `future` 都返回 `Pending`，则返回 `Pending` 让上层继续调度

实际实现很精巧，但理解其“同时轮询 + 单赢取消”的本质就够了。

# 六、常见模式与最佳实践

## 模式 1：带超时的通道接收

```rust
select! {
    msg = rx.recv() => { /* 处理消息 */ }
    _ = tokio::time::sleep(Duration::from_secs(5)) => { /* 超时处理 */ }
}
```

## 模式 2：循环接收多条消息（同时监听关闭信号）

```rust
loop {
    select! {
        msg = rx.recv() => {
            match msg {
                Some(m) => process(m).await,
                None => break, // 通道关闭
            }
        }
        _ = shutdown_signal.recv() => {
            // 开始优雅关闭
            break;
        }
    }
}
```

## 模式 3：多通道优先级处理（biased）

```rust
select! {
    biased;
    urgent = urgent_rx.recv() => handle_urgent(urgent),
    normal = normal_rx.recv() => handle_normal(normal),
}
```

## 模式 4：尝试在 select 内执行异步操作

你不能在 `handler` 中直接 `.await`，因为 `handler` 是同步代码。如果需要基于分支结果再启动异步操作，可以：

```rust
select! {
    token = token_rx.recv() => {
        // 启动一个新任务，不阻塞当前 select 返回
        tokio::spawn(async move {
            do_something_with_token(token).await;
        });
    }
    // ...
}
```

或者让整个 `select!` 返回一个值，在外部 `.await`。

## 七、与 `join!` 的区别

宏	行为	取消策略
`select!`	等待首个就绪，取消其余	选中后立即 `drop` 其他 `future`
`join!`	等待全部完成	中途出现 `panic` 会取消其他 `future`
`try_join!`	等待全部完成，遇到 `Err` 即提前返回错误，取消其余	同 `join`，但遇到错误立即返回

# 八、常见陷阱

- 忘记 cancel safety —— 在 `select!` 中直接使用 `read_exact` 这类未来导致数据丢失。

- 模式匹配永不成功 —— 如果 `Ok(v)` 模式永远无法匹配，因为 `future` 只会返回 `Err`，该分支就永不被选中。

- 在循环中使用 `biased` 且优先级高的分支始终就绪，导致低优先级饥饿。

- 在 `handler` 中执行耗时同步操作 —— 会阻塞当前任务线程，应该移到 `spawn_blocking` 或拆成小步。

- `select!` 中变量所有权 —— 每个分支的 `future` 可以借用外部变量，但要遵守 Rust 的借用规则，宏会帮你处理，但遇到复杂的 `async move` 块可能会有所有权冲突。

# 总结

`tokio::select!` 是编写高并发、响应式异步程序的核心宏。它能优雅地编排多个异步事件流，但必须深刻理解“cancel safety”才能避免隐藏的数据错误。日常使用记住三点：

- 优先使用 cancel-safe 的操作（通道接收、计时器、信号等）

- 对有副作用的 I/O 操作，要么用 `read` 等安全方法，要么 `spawn` 出去

- 结合 biased 与模式匹配实现清晰的优先级控制

理解了这些，你就能充分发挥 `select!` 的能力，写出既简洁又健壮的异步逻辑。
