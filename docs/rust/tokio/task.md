state创建时默认3个ref count
D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\task\state.rs
```
pub(super) fn new() -> State
```

为什么？
## 创建新task的时候，其实创建了3个handle：Task、Notified、JoinHandle
D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\task\mod.rs
```Rust
fn new_task<T, S>(
    task: T,
    scheduler: S,
    id: Id,
) -> (Task<S>, Notified<S>, JoinHandle<T::Output>)
where
    S: Schedule,
    T: Future + 'static,
    T::Output: 'static,
```

Task实现了Drop，drop时会dec ref
```Rust
impl<S: 'static> Drop for Task<S> {
    fn drop(&mut self) {
        // Decrement the ref count
        if self.header().state.ref_dec() {
            // Deallocate if this is the final ref count
            self.raw.dealloc();
        }
    }
}
```

Notified没实现Drop，但wrap了Task

JoinHandle也实现了Drop
D:\cargo_home\registry\src\index.crates.io-6f17d22bba15001f\tokio-1.36.0\src\runtime\task\join.rs
```Rust
impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        if self.raw.state().drop_join_handle_fast().is_ok() {
            return;
        }

        self.raw.drop_join_handle_slow();
    }
}
```

创建UnownedTask时，forget了Task，Notified，但在Drop时，ref count - 2
```Rust
pub(crate) fn unowned<T, S>(task: T, scheduler: S, id: Id) -> (UnownedTask<S>, JoinHandle<T::Output>)
where
    S: Schedule,
    T: Send + Future + 'static,
    T::Output: Send + 'static,

impl<S: 'static> Drop for UnownedTask<S> {
    fn drop(&mut self) {
        // Decrement the ref count
        if self.raw.header().state.ref_dec_twice() {
            // Deallocate if this is the final ref count
            self.raw.dealloc();
        }
    }
}
```