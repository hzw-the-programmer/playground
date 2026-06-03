```rust
ptr::write(&raw mut (*inner).value, data);
```

这行代码在堆上分配的 `RcInner<T>` 的 `value` 字段位置写入 `data`，完成 `T` 的原地构造。它背后涉及裸指针操作、未初始化内存处理以及 Rust 的安全边界，我们逐层详解。

# 1. 为什么不能直接赋值？

你可能第一反应是

```rust
(*inner).value = value; // 编译不过！
```

这样写会触发编译错误，因为 Rust 不允许通过裸指针直接字段赋值（需要解引用为左值，而 `*inner` 要求 `T` 是 `Sized` 且此处仍被视为移动语义，且需写入已初始化位置）。即使语法允许，它也隐含了读取和覆盖的操作，而不符合此处“位置未初始化”的前提。

如果写成：

```rust
let mut_ref = &mut (*inner).value;
*mut_ref = value;
```

这是 **未定义行为**：`&mut (*inner).value` 会创建一个对未初始化内存的引用，而 Rust 规定引用必须指向已初始化的数据。这破坏了 Rust 的安全假设。

所以我们需要一个能**直接写入未初始化内存**、**不创建引用**、也**不触发旧值析构**的工具，这就是 `ptr::write`。

# 2. `&raw mut` 语法的作用

`&raw mut (*inner).value` 使用了 raw reference 操作符（`&raw mut`），它直接将字段路径解析为一个裸指针（`*mut T`），全程不生成中间引用。

传统写法 `ptr::addr_of_mut!((*inner).value)` 是一样的效果（`addr_of_mut!` 是宏，展开后等价）。

`&raw mut` 是 Rust 2021 版引入的语法，更直观。

为什么需要这个操作符？
因为 `(*inner).value` 是一个字段投影。如果直接取地址：

```rust
&raw mut (*inner).value
```

Rust 编译器明白我们要的是字段的裸指针，它不会插入任何对 `*inner` 或 `value` 的中间引用，因此允许 `inner` 指向的内存尚未初始化。这是 `&raw mut` 的关键用途：安全地取得未初始化字段的指针。

# 3. `ptr::write` 做了什么？

`std::ptr::write` 的签名：

```rust
pub unsafe fn write<T>(dst: *mut T, src: T)
```

它执行的动作等价于：

```rust
*dst = src; // 直接覆写内存
```

但 不会 对 `dst` 原本指向的值进行 `drop` 或读取，仅仅将 `src` 的位模式写入目标地址。

它适合这种场景：

- `dst` 指向的内存是未初始化的，或者之前的值已经被 `ptr::read` 转移走，无需析构。

- 你不想触发旧值的 `Drop` 实现。

- `dst` 是合法分配的、对齐正确、可写入的。

因此这里 `ptr::write(&raw mut (*inner).value, value)` 把 `value` 移动到了堆内存的 `value` 字段中，完成初始化。

# 4. 安全性分析（unsafe 块的义务）

`ptr::write` 本身是 `unsafe` 函数，调用者必须保证：

- `dst` 指向的内存是有效分配的，且至少有 `size_of::<T>()` 字节大小，对齐正确。
  → `inner` 来自 `Box::new_uninit`，内存足够大，对齐由 `RcInner<T>` 保证，满足。

- `dst` 指向的内存没有同时被其他指针访问（别名规则）。
   → 此时我们拥有 `inner` 的唯一所有权，没有其他引用，符合。

- 写入一个 `T` 之后，`dst` 对应内存必须被当作已初始化看待，后续不能再重复 `ptr::write` 初始化它，除非先 `ptr::read` 移走或手动 `drop`。
  → 我们只写一次，之后 `Rc` 会负责在引用计数降为零时 `drop` 这个 `value`。

- 如果 `T` 不是 `Copy`，`src` 将被移动，`ptr::write` 不会复制它。
  → 这里 `value` 被移动进堆内存，符合所有权。

# 5. 对比其他方式

方式	可行性	问题
`(*inner).value = value;`	❌	语法不允许（裸指针解引用后赋值），且语义不安全
`&mut (*inner).value = value;`	❌	创建对未初始化内存的引用 → UB
`*(&raw mut (*inner).value) = value;`	⚠️	仍是一个赋值，会要求该位置已初始化（写操作本身可能假定先读取旧值）
`ptr::write(&raw mut (*inner).value, value)`	✅	直接写入位模式，无析构、无读取，适用于未初始化内存

# 总结

`ptr::write(&raw mut (*inner).value, data)` 是一句浓缩了 Rust 裸指针和未初始化内存操作精华的代码：

- `inner` 是裸指针，指向堆上未初始化的 `RcInner<T>`；

- `&raw mut (*inner).value` 安全地获取 `value` 字段的裸指针，不产生引用；

- `ptr::write` 直接将 `data` 移动到该内存位置，不读取旧值、不触发析构，完成初始化。

这正是 Rust 实现 `Rc`、`Arc`、`Vec` 等智能指针和容器时必须使用的底层原语，体现了 Rust 在不牺牲安全性的前提下，对内存布局的极致掌控。
