# 一、做了什么

Next 是把 “异步流（Stream）的下一个元素” 包装成一个可等待的 Future。

- 核心作用：调用 `stream.next().await` 时，异步拉取流的下一个元素，返回 `Some(item)` 或 `None`（流结束）。
- 一句话：Stream → 包装 → Future，用于 await 拿流的下一项。

# 二、用了什么技术

1. Future trait 实现：Next 是一个 Future，`Output=Option<St::Item>`；`poll` 时调用内层流的 `poll_next_unpin`。
2. Unpin 约束：要求流 `St: Unpin`，让 Next 可安全引用并多次 `poll`。
3. Pin + Context 调度：用 `Pin<&mut Self>` 管理自引用，`Context` 传递任务唤醒器，实现异步调度。
4. FusedFuture 可选实现：流是 `FusedStream` 时，`Next` 可提前标记终止，避免无效 `poll`。

# 三、最简总结

- 功能：将 `Stream` 的 “下一项” 转为 `Future`，支持 `.await` 异步获取。
- 技术：Rust 异步核心（Future/Poll/Pin/Context）+ 流适配 + Unpin 安全约束。
