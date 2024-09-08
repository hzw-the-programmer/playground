值的类型实现了Try，那么可以对该值使用？操作符，如：
```rust
fn test() -> Result<Foo, Bar> {
    let r = Ok(Foo);
    // let r = Err(Bar);
    let r = r?;
    Ok(r)
}
```
？操作符可以改写为如下代码：
```rust
fn test() -> Result<Foo, Bar> {
    let r = Ok(Foo);
    
    // let r = r?;
    let r = match r.branch() {
        ControlFlow::Continue(c) => c,
        ControlFlow::Break(b) => return FromResidual::from_residual(b),
    }
    
    // Ok(r)
    Try::from_output(r)
}
```
Result的Try实现如下：
```rust
impl<T, E> ops::Try for Result<T, E> {
    type Output = T;
    type Residual = Result<convert::Infallible, E>;
    fn from_output(output: Self::Output) -> Self {
        Ok(output)
    }
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}
impl<T, E, F: From<E>> ops::FromResidual<Result<convert::Infallible, E>> for Result<T, F> {
    fn from_residual(residual: Result<convert::Infallible, E>) -> Self {
        match residual {
            Err(e) => Err(From::from(e)),
        }
    }
}
```
特别注意from_residual中，From::from(e)，这是实现错误传递的关键。
