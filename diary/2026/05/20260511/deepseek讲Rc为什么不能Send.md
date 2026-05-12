为什么 Rc<T> 不能跨线程移动

---

在 Rust 中，Rc<T> 之所以不能跨线程移动，根本原因是为了保证并发安全和避免数据竞争。换一种说法就是：Rc<T> 没有实现 Send trait，而 Send 是允许类型在线程间转移所有权的标记 trait。

# 1. 关键原因：非原子的引用计数

Rc<T> 内部使用非原子操作来维护引用计数。每次调用 Rc::clone 或丢弃一个 Rc 时，只是对内部的引用计数字段进行简单的 +1 / -1 操作，比如：

```rust
// 简化模拟的内部操作
self.strong_count += 1;   // 增加计数
self.strong_count -= 1;   // 减少计数
```

这种整数运算在单线程下完全正确，但在多线程下，读-改-写不是原子的：

- 线程 A 读取计数（比如 1）
- 线程 B 同时读取计数（也是 1）
- A 写回 2，B 也写回 2
- 本应是 3，结果变成了 2，造成“引用计数丢失”，最终导致对象被提前释放（use-after-free）或内存泄漏。

这就是经典的数据竞争（data race），属于未定义行为（UB）。

# 2. Rc<T> 故意不实现 Send

Rust 通过类型系统在编译期阻止这类危险。Rc<T> 的设计者明确没有为其实现 Send（以及 Sync），因此编译器会在你尝试将 Rc 发送给另一线程时直接报错，例如：

```rust
use std::rc::Rc;
use std::thread;

let rc = Rc::new(42);
thread::spawn(move || {
    println!("{}", rc);
}).join();
```

编译错误类似于：

```rust
error[E0277]: `Rc<i32>` cannot be sent between threads safely
   --> src/main.rs:...
    |
    | thread::spawn(move || {
    |            ^^^^^^^^^^ `Rc<i32>` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `Rc<i32>`
```

这说明 Rc 不能在 thread::spawn 这样的上下文中通过 move 转移所有权到另一个线程。

# 3. 如果你需要在多线程间共享所有权

对应的线程安全版本是 Arc<T>（Atomic Reference Counted）。它与 Rc<T> 功能类似，但使用 原子操作（如 atomic increment/decrement）来管理计数，虽然有一定性能开销，但保证了多线程安全，因此 Arc<T> 实现了 Send 和 Sync（当 T 满足相应条件时）。

# 4. 补充：移动后的“遗留线程”问题

也许你会想：“我只是把 Rc 移动到另一个线程，原线程不再碰它，应该安全吧？”
问题是：Rc 除了 Send，还有 Sync（共享引用安全）的缺失，它通常与 RefCell 等非线程安全内部可变性类型组合使用。但更直接的原因是 Rc 的引用计数本身就不是线程安全的，即使只有一个线程拥有所有权，也可能存在另外的弱引用（Weak）被留在原线程的某个结构里，导致计数操作跨线程发生。Rust 选择了保守且可靠的方式：直接不实现 Send，从源头禁止任何跨线程移动。

# 总结一下：

- Rc<T> 使用非原子引用计数，多线程下增减计数会产生数据竞争。
- 因此它不实现 Send，无法转移所有权到其他线程。
- 若需跨线程共享所有权，请使用 Arc<T>（原子引用计数）。
