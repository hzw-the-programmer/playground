RawTask是Copy
D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\task\raw.rs
```Rust
#[derive(Clone)]
pub(crate) struct RawTask {
    ptr: NonNull<Header>,
}
impl Copy for RawTask {}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ptr\non_null.rs
```Rust
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}
impl<T: ?Sized> Clone for NonNull<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized> Copy for NonNull<T> {}
```
