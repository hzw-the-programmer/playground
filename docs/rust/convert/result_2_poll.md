D:\github\tower-rs\tower\tower\src\util\service_fn.rs
```Rust
impl<T, F, Request, R, E> Service<Request> for ServiceFn<T>
where
    T: FnMut(Request) -> F,
    F: Future<Output = Result<R, E>>,
{
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), E>> {
        Ok(()).into()
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\task\poll.rs
```Rust
impl<T> From<T> for Poll<T> {
    fn from(t: T) -> Poll<T> {
        Poll::Ready(t)
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\convert\mod.rs
```Rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```
