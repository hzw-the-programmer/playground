```rust
pub fn and<U>(self, res: Result<U, E>) -> Result<U, E> {
    match self {
        Ok(_) => res,
        Err(e) => Err(e),
    }
}
```
# and_then

值为Err(e)，and短路返回Err(e)
值为Ok(t)，返回取决于op，op可以返回Ok(u)，也可以返回Err(e)
op有转换(map)值得功能
和map函数区别在于回调函数
map: F: FnOnce(T) -> U
map 值是Ok时，返回值也是Ok
and_then 值是Ok时，返回值可为Ok或Err

```rust
pub fn and_then<U, F: FnOnce(T) -> Result<U, E>>(self, op: F) -> Result<U, E> {
    match self {
        Ok(t) => op(t),
        Err(e) => Err(e),
    }
}
```

```rust
pub fn or<F>(self, res: Result<T, F>) -> Result<T, F> {
    match self {
        Ok(v) => Ok(v),
        Err(_) => res,
    }
}
```

```rust
pub fn or_else<F, O: FnOnce(E) -> Result<T, F>>(self, op: O) -> Result<T, F> {
    match self {
        Ok(t) => Ok(t),
        Err(e) => op(e),
    }
}
```

```rust
pub fn unwrap_or(self, default: T) -> T {
    match self {
        Ok(t) => t,
        Err(_) => default,
    }
}
```

```rust
pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
    match self {
        Ok(t) => t,
        Err(e) => op(e),
    }
}
```
