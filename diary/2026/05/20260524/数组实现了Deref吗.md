最终正确结论

- [T; N] 数组类型 没有 实现 Deref trait！
- &[T; N] 也没有实现 Deref！

那为什么 &[T; N] 能自动变成 &[T]？

这不是 Deref，而是 Rust 编译器内置的、专属数组的强制转换规则：

数组引用自动解引用为切片（array to slice coercion）

这是语言内置能力，不是靠 trait 实现的！

Rust 专门为数组设计了一条强制类型转换规则：

```text
&[T; N]    → 自动强制转换 →    &[T]
&mut [T; N] → 自动强制转换 → &mut [T]
```

这叫 unsized coercion / array pointer unsizing

和 Deref 无关，和 trait 无关。

❌ 不是 Deref

```rust
let arr = [1,2,3];
let s: &[i32] = &arr; // 内置强制转换，不是 Deref
```

✅ 才是 Deref

```rust
let v = vec![1,2,3];
let s: &[i32] = &v;   // Vec 实现了 Deref
```

总结（最准确版本）

- [T; N] 没有实现 Deref
- &[T; N] 没有实现 Deref
- 数组→切片靠编译器内置强制转换
- 只有 Vec/String/Box 这类才是靠 Deref
