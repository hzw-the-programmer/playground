Rc 实现了 fmt::Pointer
alloc\src\rc.rs
```rust
impl<T: ?Sized, A: Allocator> fmt::Pointer for Rc<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&(&**self as *const T), f)
    }
}
```

&(&**self as *const T)怎么理解？

self: &Rc
*self: Rc
**self: T
&**self: &T
&**self as *const T: *const T
&(&**self as *const T): &*constT

参看：workspace\examples\deref\src\test5.rs

那么 *const T 必定实现 fmt::Pointer。
core\src\fmt\mod.rs
```rust
impl<T: ?Sized> Pointer for *const T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Cast is needed here because `.expose_addr()` requires `T: Sized`.
        pointer_fmt_inner((*self as *const ()).expose_addr(), f)
    }
}
```
