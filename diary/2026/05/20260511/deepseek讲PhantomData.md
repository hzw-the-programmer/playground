pub struct Empty<T> {
    _phantom: PhantomData<T>,
}
这里 PhantomData解决什么问题

---

`PhantomData<T>` 在 `Empty<T>` 中主要解决三个问题：

# 🏷️ 让 T 被“使用”

Rust 不允许存在未被使用的类型参数。`Empty` 不存任何值，若无 `_phantom`，`T` 形参纯属多余，编译器会直接报错。`PhantomData<T>` 是最轻量的“占位符”，告诉编译器“这个类型逻辑上依赖于 `T`”。

# 🔄 控制变型（Variance）

`PhantomData<T>` 默认让类型对 `T` 协变（跟作用在具体的 `T` 值上一样）。这很自然：假如 `A` 是 `B` 的子类型，那么 `Empty<A>` 也能当成 `Empty<B>` 用。换成 `PhantomData<fn(T)>` 就会逆变，但这里显然需要协变。

# 🧵 绑定自动 trait

`Send`、`Sync` 等自动推导 trait 会穿透到字段。`PhantomData<T>` 虽不占内存，但编译器推导时会把它当作“逻辑上持有 `T`”，所以 `Empty<T>` 是否 `Send/Sync` 完全由 `T` 决定。这保证了 `Empty<Rc<()>>` 不会是 `Send`，行为与实际持有 `T` 的类型一致。

一句话概括：`PhantomData<T>` **让一个零大小的空结构体在类型系统层面“表现得像是真的包含了 `T`”，从而满足编译器对类型参数使用的要求，并保持正确的变型和线程安全推导**。
