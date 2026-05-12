详解 Send/Sync 推导

---

我用最清晰、最容易理解、最贴近底层的方式，给你彻底讲透 Rust 中 Send/Sync 是如何自动推导的。
你看完就能闭着眼睛判断任何结构体是不是 Send + Sync。

# 一、先记住 2 句宇宙真理（90% 场景都够用）

1. 如果所有字段都是 Send → 整个结构体就是 Send
2. 如果所有字段都是 Sync → 整个结构体就是 Sync

反之：只要有一个字段不是 Send/Sync → 整个结构体就不是 Send/Sync

---

# 二、什么是 Send / Sync（超简版）

- Send: 能把值转移到另一个线程（安全）
- Sync: 能把 &T 共享给多个线程（安全）

---

# 三、自动推导规则（编译器就是这么干的）

## 规则 1：默认自动实现

如果你的结构体：
没有手动写 unsafe impl Send / Sync
所有字段都满足 Send/Sync
→ 编译器自动给你实现

## 规则 2：只要有一个字段不满足 → 整个结构体不满足

例子：

```rust
struct Foo {
    a: Rc<i32>,    // ❌ Rc 既不是 Send 也不是 Sync
    b: i32,         // ✅ Send + Sync
}
```

结果：
- Foo 不是 Send
- Foo 不是 Sync

## 规则 3：裸指针 *const T / *mut T → 默认！Send + !Sync

因为编译器无法保证安全。

## 规则 4：PhantomData<T> 完全跟随 T

```rust
struct Empty<T> {
    _phantom: PhantomData<T>, // Send/Sync = T 的 Send/Sync
}
```

这就是你刚才 Empty 结构体的关键！

---

# 四、哪些常见类型是 Send / Sync？

✅ Send + Sync（绝大多数）
- i32, bool, f32, String, Vec
- &T（如果 T: Sync）
- Arc<T>, Mutex<T>
- Box<T>, VecDeque<T>
- 所有基础类型、标准集合

❌ !Send（不能跨线程移动）
- Rc<T>
- 某些自引用类型
- 某些 GPU 句柄

❌ !Sync（不能跨线程共享）
- Rc<T>
- Cell<T>, RefCell<T>
- *mut T 裸指针
- UnsafeCell<T>

---

# 五、最关键：PhantomData<T> 如何影响 Send/Sync？

结论：
PhantomData<T> 本身不做任何事，它的 Send/Sync 完全等于 T
编译器会把它当作 “拥有一个 T” 来推导。
例子 1：T = i32（Send + Sync）
```rust
struct Empty<T> { _phantom: PhantomData<T> }
```

Empty<i32>
→ 自动：Send + Sync

例子 2：T = Rc<i32>（!Send + !Sync）
Empty<Rc<i32>>
→ 自动：!Send + !Sync

例子 3：T = Arc<Mutex<i32>>（Send + Sync）
→ 自动：Send + Sync

# 六、完整推导过程（编译器内部执行步骤）

给你看编译器如何推导 Empty<T>：

```rust
pub struct Empty<T> {
    _phantom: PhantomData<T>,
}
```

- 查看所有字段：只有 PhantomData<T>
- PhantomData<T> 的 Send/Sync = T 的 Send/Sync
- 所以：
  + Empty<T>: Send 当且仅当 T: Send
  + Empty<T>: Sync 当且仅当 T: Sync

---

# 七、超级总结（背会这一段就够）

1. Rust Send/Sync 推导规则：
2. 全字段满足 → 结构体满足
3. 一个不满足 → 结构体不满足
4. PhantomData<T> 完全继承 T 的 Send/Sync
5. 默认自动推导，无需手动写
6. 只有需要打破规则时，才手动写 unsafe impl

---

# 八、你刚才的 Empty<T> 到底做了什么？

```rust
pub struct Empty<T> {
    _phantom: PhantomData<T>,
}
```

它让：
- Empty<T> 像 T 一样 Send/Sync
- 但不存储任何数据
- 编译安全
- 0 运行时开销

这就是 Rust 类型系统的零成本抽象。

---

如果你愿意，我可以出 3 道超级经典的 Send/Sync 判断题，让你彻底巩固这个知识点！
