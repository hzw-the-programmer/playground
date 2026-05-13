在 unsafe 下, 把 &T 强制转换为 *mut T 赋值，不就实现了改变？

---

灵魂拷问！问到了 UnsafeCell 存在的真正本质

你说的完全正确：

在 unsafe 下，把 &T 强转成 *mut T 然后赋值，语法上确实能改。

但我现在告诉你最核心、99% 人不知道的真相：

这样做 = 未定义行为（UB）

只有用 UnsafeCell 才是 合法、定义良好 的

我用最直白、最硬核的方式给你讲透👇

---

1. 你说的写法（强转 &T → *mut T）

```rust
#[allow(invalid_reference_casting)]

fn bad_modify(x: &i32) {
    unsafe {
        let ptr = x as *const i32 as *mut i32;
        // error: assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
        *ptr = 100; // ❌ 未定义行为！
    }
}

fn main() {
    let a = 5;
    bad_modify(&a);
}
```

编译报错：error: assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
加上 #[allow(invalid_reference_casting)]，可编译过，但是未定义行为。

---

2. 为什么强转不行？（编译器魔法：noalias）

Rust 编译器对 &T 有一个铁律：

- &T = 指向的值不会改变
- 编译器会加上 noalias 优化：
- 认为值不会变
- 可以缓存到寄存器
- 可以重排指令
- 可以直接删去死代码

你偷偷修改，编译器不知情 → 程序行为随机、崩溃、乱输出。
这就是 未定义行为（Undefined Behavior）

---

3. UnsafeCell 做了什么？

UnsafeCell 告诉编译器：

> 这个值可能会变，不要优化我！不要加 noalias！

它是 Rust 唯一官方认可 的：
允许 “共享可读、同时可写” 的类型
只有 UnsafeCell::get () 返回的 *mut T 才是合法可写的。

---

4. 最关键结论（背下来）

方式	行为	结果
&T → *mut T 强转	绕过编译器修改	未定义行为（崩溃 / 乱码 / 优化错误）
UnsafeCell::get()	编译器认可的后门	完全合法、安全、可移植

---

5. 标准库原话（The Rustonomicon）

> The only legal way to obtain a mutable pointer to a shared value is through UnsafeCell.
> 唯一合法的获取共享值可变指针的方式，就是 UnsafeCell。

---

6. 用 UnsafeCell 的正确写法（合法）

```rust
use std::cell::UnsafeCell;

fn good_modify(x: &UnsafeCell<i32>) {
    unsafe {
        let ptr = x.get(); // ✅ 编译器认可
        *ptr = 100;
    }
}

fn main() {
    let x = UnsafeCell::new(5);
    good_modify(&x);
}
```

---

7. 最终极总结（一句话）

强转 &T → *mut T 能修改，但编译器会误优化，导致崩溃。
UnsafeCell 是唯一让编译器不优化、承认可写的机制。
