什么样的stream实现Unpin？

---

简单来说，**绝大多数不含自引用结构的 `Stream` 都自动实现了 `Unpin`**。具体判断标准如下：

# ✅ 会自动实现 `Unpin` 的 `Stream`

只要类型本身不依赖于固定的内存地址，编译器就会自动为它实现 Unpin，这包括：

1. **标准库基础类型与集合**
   例如 `i32`、`String`、`Vec<T>`、`HashMap<K, V>` 等，它们都实现了 `Unpin`。

2. **所有 `Unpin` 字段组成的结构体或枚举**
   如果你自定义的 `MyStream` 结构体内每个字段都是 `Unpin`，那 `MyStream` 本身也是 `Unpin`。
   比如：
   ```rust
   struct Counter { count: u64 }
   impl Stream for Counter { /* ... */ }
   ```
   这里 `Counter` 就是 `Unpin` 的。

3. **`futures-rs` 组合器，当内部流为 `Unpin` 时**
   如 `Map<St, F>`、`Filter<St, F>` 等，只要被包装的流 `St` 是 `Unpin`，整个组合器也是 `Unpin`。

4. **通过 `Box<dyn Stream>` 或 `Pin<Box<T>>` 包装的流**
   `Box<T>` 总是 `Unpin`，即使 `T: !Unpin`，因此 `Box::pin(my_stream)` 后的智能指针本身是 `Unpin` 的，可以直接调用 `.next()`。

# ❌ 不会实现 `Unpin` 的 `Stream`

这些流内部可能包含指向自身字段的指针，因此被标记为 `!Unpin`：

1. **`async` 块或生成器创建的流（例如通过 `async-stream` crate）**
   它们编译成匿名的状态机，在 `.await` 点之间可能保存对自身局部变量的引用，因此不是 `Unpin`。
   这也是你用 `stream.next()` 时遇到编译错误最常见的原因。

2. **手动包含 `PhantomPinned` 或用 `unsafe` 实现 `!Unpin` 的类型**
   当你显式插入 `PhantomPinned` 字段，或通过负实现 `impl !Unpin for MyStream {}` 主动放弃 `Unpin`。

# 🔧 实际判断方法

当你写这样一行代码时：

```rust
stream.next().await; // 要求 Stream: Unpin
```

- 如果编译通过 → 该流实现了 `Unpin`。
- 如果编译报错，提示类似 `the traitUnpinis not implemented` → 需要固定它，例如：
  ```rust
  let mut stream = Box::pin(my_async_stream);
  stream.next().await; // Box 是 Unpin，可以调用
  ```

总结：凡是可以安全移动而不破坏内部指针的结构，就是 Unpin 的 Stream。绝大多数手写的、非自引用的流都满足这一点；只有异步生成器或刻意设计的自引用结构才不是 Unpin。
