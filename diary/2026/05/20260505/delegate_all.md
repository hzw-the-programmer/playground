D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\futures-util-0.3.32\src\future\try_future\mod.rs

```rust
delegate_all!(
    /// Future for the [`map_ok`](TryFutureExt::map_ok) method.
    MapOk<Fut, F>(
        Map<IntoFuture<Fut>, MapOkFn<F>>
    ): Debug + Future + FusedFuture + New[|x: Fut, f: F| Map::new(IntoFuture::new(x), map_ok_fn(f))]
);
```

根据定义：
D:\cargo_home\registry\src\index.crates.io-1949cf8c6b5b557f\futures-util-0.3.32\src\lib.rs

生成

`MapOk` 结构体，它的 `inner` 字段是 `Map<IntoFuture<Fut>, MapOkFn<F>>` 类型。
实现 `Debug`, `Future`, `FusedFuture`。
实现 `new` 方法
