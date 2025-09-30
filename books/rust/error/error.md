实现 Error 的话，必须先实现 Debug 和 Display。
core\src\error.rs
```rust
pub trait Error: Debug + Display {
}
```

实现 Error 的例子：
tower\tower\src\timeout\error.rs
```rust
#[derive(Debug, Default)]
pub struct Elapsed(pub(super) ());

impl fmt::Display for Elapsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timed out")
    }
}

impl error::Error for Elapsed {}
```
