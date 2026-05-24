我给你最全、最实用、最不容易踩坑的 4 种判断 Rust Debug / Release 模式的方法，直接复制就能用。

一、最常用：编译期判断（#[cfg]）

1. 判断是否为 Debug 模式

```rust
#[cfg(debug_assertions)]
println!("现在是 Debug 模式！");
```

2. 判断是否为 Release 模式

```rust
#[cfg(not(debug_assertions))]
println!("现在是 Release 模式！");
```

二、运行时判断（代码里 if 判断）

```rust
fn main() {
    if cfg!(debug_assertions) {
        println!("运行时：Debug 模式");
    } else {
        println!("运行时：Release 模式");
    }
}
```

三、最权威：Cargo 编译命令

```bash
# Debug（默认）
cargo build

# Release
cargo build --release
```

四、通过输出目录判断

- Debug 产物：target/debug/xxx
- Release 产物：target/release/xxx

五、核心原理（必须懂）

Rust 没有官方的 debug / release 配置标记，

唯一标准就是：debug_assertions 是否开启

- Debug → debug_assertions = enabled
- Release → debug_assertions = disabled

所以：

- #[cfg(debug_assertions)] = Debug
- #[cfg(not(debug_assertions))] = Release

六、真实工程常用写法

1. 只在 Debug 打印日志

```rust
#[cfg(debug_assertions)]
macro_rules! debug_log {
    ($($tt:tt)*) => { println!($($tt)*) };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_log {
    ($($tt:tt)*) => {};
}
```

2. Debug 开启检查，Release 关闭

```rust
#[cfg(debug_assertions)]
fn check_something() {
    // 严格检查
}

#[cfg(not(debug_assertions))]
fn check_something() {}
```

3. Release 才做性能优化

```rust
#[cfg(not(debug_assertions))]
#[inline(always)]
fn fast_path() { ... }
```

七、一句话终极总结

判断 Debug / Release 唯一标准：cfg(debug_assertions)

- 开启 = Debug
- 没开启 = Release
