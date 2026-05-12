用简介的语言描述下 futures_util::stream::stream::Next 做了什么事？用了什么技术？

---

`futures_util::stream::stream::Next` 的核心作用是将对 `Stream` 的轮询操作封装成一个 `Future`。简单来说，它在 `.await` 调用点与底层 `Stream` 实现之间建立起一座异步桥梁。其背后的核心技术主要是**异步适配器模式**、**`Pin` 投影**与**取消安全**。

# ⚙️ 实现细节

`Next` 的实现很简洁，核心逻辑如下：

1. **结构定义**：它是一个泛型结构体，持有一个对底层 `Stream` 的可变引用 `stream: &'a mut St`。

2. **智能构造**：其 `new` 方法接收一个对 `Stream` 的可变引用并进行存储，要求 `Stream` 实现 `Unpin` trait。

3. **异步桥梁 `Future` 实现**：为 `Next` 实现 `Future` trait 是其存在的根本意义。在 `poll` 方法中，它将自身 `Pin` 住，并将 `Context` 转发给内部 `stream` 的 `poll_next_unpin` 方法，从而将对 `Stream` 的轮询转换为了对 `Future` 的轮询。

# 🛠️ 关键技术

- **异步适配器模式 (Async Adapter Pattern)**：`Next` 本身不直接处理数据流的获取，而是作为一个中间层，将对 `Future::poll` 的调用转发给底层的 `Stream`。这种模式允许开发者在不修改原有异步原语的情况下，通过组合实现更复杂的行为，是 `futures-rs` 库中构建异步组合子的核心模式之一。

- `Pin` 与 `Unpin`：`Pin<&mut Self>` 保证了 `Next` 在内存中的地址不会改变，这是调用底层 `Stream` 的 `poll_next_unpin` 方法的前提。同时，`Next` 要求其包装的 `Stream` 实现 `Unpin`，这是因为方法只持有了一个可变引用，而有些 `Stream`（如通过 `async` 生成器创建的）内部可能包含自引用，需要额外用 `Box::pin` 固定才能安全地使用 `next()`。

- **取消安全 (Cancel Safety)**：`Next` 被设计为“取消安全”的。代码注释明确提到，即使 `Next` future 被丢弃，也不会丢失任何数据，因为它只持有对底层流的引用。这保证了在异步取消发生时，程序逻辑依然正确。

# 💡 补充说明

除了基础的 `Next`，`futures-util` 库还提供了 `NextIf` 等类似的结构体，用于需要根据条件获取下一个元素的场景。

`futures_util::stream::stream::Next` 通过简洁的适配器模式，在完成类型转换的同时，巧妙地处理了异步编程中内存管理（`Pin`）和控制流（取消安全）的关键问题。
