```rust
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}

pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
```

# 为什么 `Cell<T>` 可以是 `Send`？

`Send` 关注的是所有权转移是否安全。

当你把一个 `Cell<T>` 移动到另一个线程时，原线程就不再拥有它了。此时，整个 `Cell<T>`（包括它包裹的 `T`）在新线程中是独占的，不存在并发修改引用计数或数据的问题。

只要内部数据 `T` 本身可以安全发送（即 `T: Send`），那么把整个 `Cell<T>` 发过去就是安全的。标准库也确实这样实现了：

```rust
unsafe impl<T: ?Sized> Send for Cell<T> where T: Send {}
```

# 为什么 `Cell<T>` 永远不是 `Sync`？

`Sync` 要求多个线程可以通过共享引用（`&T`）同时访问同一个值。

`Cell<T>` 的核心功能就是通过共享引用（`&self`）来修改内部值（`set` 方法），这如果在多线程中同时发生就是数据竞争。因此标准库明确标记：

```rust
impl<T: ?Sized> !Sync for Cell<T> {}
```

# 和 `Rc<T>` 的对比

你可能会觉得矛盾：`Rc<T>` 也有内部数据，为什么移动所有权就不行？

区别在于 `Rc` 在移动后，原来的线程可能还持有它的克隆。

你 `clone` 一个 `Rc` 后，新旧两个 `Rc` 指向同一块引用计数。

即使你把其中一个 `Rc` 发送到新线程，老线程里的另一个 `Rc` 仍然可以同时对引用计数执行 +1/-1，这就产生了非原子操作上的数据竞争。

而 `Cell<T>` 被移动后，原线程不再有任何指向它的引用，独占了，因此安全。

`Cell<T>` 可以安全地送给另一个线程（如果 `T: Send`），但不能借给多个线程共享（不是 `Sync`）。
