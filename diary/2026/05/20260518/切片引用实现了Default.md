看 `httpparse` 源码时，发现 `mem::take` 的一个用法：

```rust
let headers = mem::take(&mut self.headers);

pub struct Request<'headers, 'buf> {
    /// The request headers.
    pub headers: &'headers mut [Header<'buf>]
}
```

这样做之所以可以是因为 `&mut [T]` 实现了 `Default`，返回可变空切片:

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\slice\mod.rs

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_default", issue = "143894")]
impl<T> const Default for &[T] {
    /// Creates an empty slice.
    fn default() -> Self {
        &[]
    }
}

#[stable(feature = "mut_slice_default", since = "1.5.0")]
#[rustc_const_unstable(feature = "const_default", issue = "143894")]
impl<T> const Default for &mut [T] {
    /// Creates a mutable empty slice.
    fn default() -> Self {
        &mut []
    }
}
```

---

D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\httparse-1.10.1\src\lib.rs

```rust
impl<'h, 'b> Request<'h, 'b> {
    fn parse_with_config(&mut self, buf: &'b [u8], config: &ParserConfig) -> Result<usize> {
        let headers = mem::take(&mut self.headers);

        /* SAFETY: see `parse_headers_iter_uninit` guarantees */
        unsafe {
            let headers: *mut [Header<'_>] = headers;
            let headers = headers as *mut [MaybeUninit<Header<'_>>];
            match self.parse_with_config_and_uninit_headers(buf, config, &mut *headers) {
                Ok(Status::Complete(idx)) => Ok(Status::Complete(idx)),
                other => {
                    // put the original headers back
                    self.headers = &mut *(headers as *mut [Header<'_>]);
                    other
                },
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Request<'headers, 'buf> {
    /// The request method, such as `GET`.
    pub method: Option<&'buf str>,
    /// The request path, such as `/about-us`.
    pub path: Option<&'buf str>,
    /// The request minor version, such as `1` for `HTTP/1.1`.
    pub version: Option<u8>,
    /// The request headers.
    pub headers: &'headers mut [Header<'buf>]
}
```
