你给的这段代码来自 Rust 标准库，它揭示了两个重要事实：

1. `&mut [T]` 确实实现了 `Default`（自 Rust 1.5.0 起就是稳定特性）。

2. 这个实现额外使用了 `const` 关键字，但目前仍不稳定。

首先纠正我们之前的判断：`&mut [T]` 已经实现了 `Default`，`default()` 返回一个**可变空切片引用 `&mut []`**。因此，对 `self.headers: &'headers mut [Header<'buf>]` 使用 `mem::take` 完全合法，它会用 `&mut []` 作为临时占位值。

---

const 在这个声明中的作用

这里的 const 将 default 方法声明为一个 常量函数（const fn），意味着它可以被在编译期求值，用于常量上下文。主要作用体现在：

1. 允许在常量/静态初始化中调用

```rust
static EMPTY_HEADERS: &mut [Header<'_>] = Default::default();
// 如果没有 const，这种静态初始化会报错，因为不能调用运行时函数
```

有了 const，Default::default() 就像 0 或 [] 一样，可以在编译期计算出结果。


2. 用于编译期断言或泛型常量

```rust
const EMPTY: &mut [i32] = <&mut [i32]>::default();
// 可以参与 const 泛型参数的计算等
```

3. 为 const_trait_impl 特性打基础

Rust 正在稳定化“常量 trait 实现”（const_trait_impl），允许在编译期执行 trait 方法。这里的 #[rustc_const_unstable] 正是该功能的实验性启用。未来，任何实现了 Default 的类型，只要其 default 是 const，都可以在编译期使用。

---

为什么现在是不稳定的（#[rustc_const_unstable]）？

虽然 Default 实现本身在 1.5.0 就稳定了，但把它标记为 const 涉及到编译器内部对 trait 方法常量求值的支持，属于较新的功能，所以用 rustc_const_unstable 属性保护。这意味着：

- 在 stable 版 中，你仍然可以正常调用 <&mut [T]>::default() 或 mem::take（它们仅依赖 Default trait，不要求 const）。

- 但如果你试图在常量上下文中使用 Default::default() 来初始化一个 &mut [T]，需要 nightly 编译器并开启 #![feature(const_default)]。

例如，下面这段代码在 stable 下会报错：

```rust
const X: &mut [u8] = Default::default(); // error: calls in constants are limited to ...
```

而启用 const_default 的 nightly 则可以顺利编译。

---

总结：const 在此处的实际意义

- 对 mem::take 的使用：无影响，因为 mem::take 只需要 T: Default，不关心 default 是不是 const。

- 对用户代码：未来可期，一旦 const_default 稳定，你就能在全局常量、静态变量、编译期函数中直接调用 Default::default() 生成 &mut []，写出更安全的零开销代码。

- 对标准库：逐步常量化，使更多基本类型可以在编译期使用，是 Rust 走向“编译期计算泛化”的重要一步。

回到 httparse，mem::take(&mut self.headers) 之所以能编译，正是因为 &mut [T] 自 1.5.0 起就是一个合法的 Default 类型，返回一个临时的 &mut [] 作为占位符。这个 const 只是额外赋予了该方法在编译期调用的可能性，与 httparse 的运行时逻辑无关。
