`ready!` 是 `futures` crate 或 `tokio` 等运行时提供的一个便利宏。标准定义类似于：

```rust
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
```

作用：**如果表达式返回 Pending，立刻从外层函数返回 Poll::Pending；如果返回 Ready(T)，则将 T 提取出来作为表达式的值**。

所以 `ready!(self.svc.poll_ready(cx))` 展开后等价于：

```rust
match self.svc.poll_ready(cx) {
    Poll::Ready(value) => value,
    Poll::Pending => return Poll::Pending,
}
```

这时 `value` 的类型就是 `poll_ready` 内部 `Result` 的部分，即 `Result<(), Self::Error>`。

接着在这个 `Result<(), Self::Error>` 上使用 `?` 运算符：

```rust
let result: Result<(), Self::Error> = ready!(self.svc.poll_ready(cx));
result?;
```

`?` 在这里等价于：

```rust
match result {
    Ok(()) => {},           // 成功，什么都不做，继续往下执行
    Err(e) => return Poll::Ready(Err(e.into())),
}
```

**关键点**：

- 如果 `poll_ready` 返回 `Ready(Err(e))`，说明 `Service` 不可用（比如负载已满、连接断开等）。

- 这个错误不能简单地变成 `Pending`，因为它是“永久失败”，需要立刻向上游传播。

- 因此 `?` 会自动将错误包装成 `Poll::Ready(Err(...))` 并从整个 `poll` 函数提前返回。

**注意**：这里的 `.into()` 调用了 `From` 转换，因为外部 `Future` 的错误类型可能和 `Service` 的错误类型不一致，需要显式转换。如果外部 `Future` 的 `Output` 就是 `Result<_, S::Error>`（例如 `OneShot` 的 `Output` 是 `Result<S::Response, S::Error>`），则 `e.into()` 就是 `identity`。

把 `ready!` 和 `?` 合并，我们得到如下完整的逻辑控制流：

```rust
ready!(svc.poll_ready(cx))?;

// 等价于：

match svc.poll_ready(cx) {
    Poll::Ready(Ok(())) => {
        // 一切正常，继续执行下面的代码
    }
    Poll::Ready(Err(e)) => {
        // 发生错误，立刻返回错误
        return Poll::Ready(Err(e.into()));
    }
    Poll::Pending => {
        // 尚未就绪，挂起等待
        return Poll::Pending;
    }
}
```

- `poll_ready` 是“背压”机制的核心，它允许 `service` 在上游压力过大时拒绝接收新请求（返回 `Pending` 或 `Err`）。通过 `ready!` 宏，我们可以简洁地处理“未就绪则挂起”的异步逻辑。

- 用 `?` 直接传播错误，避免手动写 `match`，代码清晰且不易出错。

- 宏在编译期展开，没有任何运行时开销。
