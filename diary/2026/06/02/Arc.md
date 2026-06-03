```rust
pub struct Arc<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
    alloc: A,
}

unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Send> Send for Arc<T, A> {}
unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Sync> Sync for Arc<T, A> {}

struct ArcInner<T: ?Sized> {
    strong: Atomic<usize>,

    // the value usize::MAX acts as a sentinel for temporarily "locking" the
    // ability to upgrade weak pointers or downgrade strong ones; this is used
    // to avoid races in `make_mut` and `get_mut`.
    weak: Atomic<usize>,

    data: T,
}
```

`Arc<T>` 要实现 `Send` 需要 `T: Send + Sync`，这源自 `Arc` 同时涉及**所有权的跨线程转移**和**多线程下的共享访问**。具体原因可以拆成两部分：

# 1. 为什么需要 `T: Send`？

当最后一个 `Arc<T>` 在某个线程被丢弃时，它会在该线程内执行 `T` 的析构，并释放堆内存。
这相当于把 `T` 的所有权转移到了那个线程。因此 `T` 必须能安全地发送到其他线程，即 `T: Send`。

补充：即使你不是显式地 `drop`，而是通过“线程结束、局部变量被析构”等方式，依然会触发 `T` 的析构。只要这个析构可能发生在与原创建线程不同的线程，就需要 `T: Send`。

# 2. 为什么需要 `T: Sync`？

`Arc<T>` 的核心用途是多线程共享同一份数据。通过 `clone` 得到的多个 `Arc<T>` 指向同一个堆内存，每个线程都可以通过 `Arc` 的 `Deref` 拿到 `&T`，即共享引用。

比如线程 A 调用 `arc.as_ref()` 得到 `&T`，同时线程 B 也拿到一个 `&T`。

这就构成了多个线程同时持有 `T` 的共享引用。

要保证这是安全的，`T` 必须支持这种并发共享访问，即 `T: Sync`。

# 3. 为什么两个条件缺一不可？

- 如果只有 `T: Send` 而没有 `Sync`，多线程共享 `&T` 时就会出问题（例如内部包含 `Cell`，允许通过共享引用修改数据）。
- 如果只有 `T: Sync` 而没有 `Send`，那么当 `Arc` 在某个线程析构时，`T` 的所有权移动行为就是非法的。

# 4. 与 `Rc<T>`、`Cell<T>` 对比

类型	Send 条件	原因
`Rc<T>`	永不 `Send`	非原子引用计数，移动后原线程仍可能有克隆，导致计数修改发生数据竞争
`Cell<T>`	`T: Send`	移动后原线程不再持有，独占安全；但共享修改导致 !Sync
`Arc<T>`	`T: Send + Sync`	原子计数保证计数安全，但 `T` 会被多线程共享（需 `Sync`）且可能在任意线程析构（需 `Send`）
