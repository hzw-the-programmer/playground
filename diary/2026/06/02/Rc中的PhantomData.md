```rust
pub struct Rc<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
> {
    ptr: NonNull<RcInner<T>>,
    phantom: PhantomData<RcInner<T>>,
    alloc: A,
}

struct RcInner<T: ?Sized> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}
```

在 `Rc` 的定义里，`phantom` 字段的类型是 `PhantomData<RcInner<T>>`，它不占实际内存，但有两个关键作用：

# 1. 声明所有权，参与 drop check

`Rc` 的 `ptr` 只是一个 `NonNull` 裸指针，编译器不会自动认为 `Rc<T>` 拥有 `RcInner<T>`（即引用计数和实际数据的结构体）。加上 `PhantomData<RcInner<T>>` 相当于告诉编译器：`Rc<T>` 在逻辑上拥有 `RcInner<T>`，`drop` 时可能会用到内部的 `T` 的析构逻辑。这让编译器的 drop check 能正确判断含有生命周期的 `T`（如 `Rc<&'a str>`）是否合法，防止悬垂引用。

# 2. 控制型变（variance）

`PhantomData<RcInner<T>>` 会使得 `Rc<T>` 对 `T` 不变（invariant）。这是因为 `RcInner<T>` 里直接存储了 `T`（所有权），本身就是不变的，`PhantomData` 把这一性质传递给了 `Rc<T>`。不变性很重要：`Rc` 内部用 `Cell` 管理引用计数，允许内部可变性，如果对 `T` 协变，就可能把 `Rc<&'short str>` 当 `Rc<&'static str>` 用，导致生命周期欺骗和悬垂指针。

简单说，`phantom` 弥补了裸指针丢失的所有权和型变信息，保证 `Rc` 的类型安全和析构正确。
