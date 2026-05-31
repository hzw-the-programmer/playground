`std::ops::Try` 是 Rust 中一个用于支撑 `?` 运算符 的 trait。它的目标是让自定义类型也能像 `Result` 和 `Option` 那样，通过 `?` 实现提前返回或错误传播。目前该 trait 仍处于 nightly 阶段（需开启 `#![feature(try_trait_v2)]`），但它的设计已经稳定并已成为 `?` 运算符的通用抽象基础。下面从设计动机、定义、核心方法、典型实现和演进历史几个维度详细展开。

---

# 1. 设计动机：让 `?` 变得可扩展

在 Rust 中，`?` 运算符可以将错误（或“中断”状态）沿着调用栈向上传播。对于 `Result<T, E>`，`?` 会在 `Err(e)` 时提前返回；对于 `Option<T>`，`?` 会在 `None` 时提前返回。但编译器对 `?` 的支持并不是硬编码在这两个类型上的——它通过一个 trait 进行抽象，这就是 `Try`。一旦你的自定义类型实现了这个 trait，它就能天然地支持 `?` 运算符，无需修改编译器。

这种设计允许实现更丰富的控制流类型，例如：

- 自定义错误类型；

- 可恢复的、包含“残差”信息的结构；

- 混合 `Result` / `Option` / `ControlFlow` 逻辑的容器。

---

# 2. 当前设计（try_trait_v2）的 trait 定义

当前 nightly 中，`std::ops::Try` 的定义如下（v2 设计）：

```rust
pub trait Try: FromResidual {
    /// 成功时产出值的类型。
    type Output;

    /// 中断时残差值的类型。
    type Residual;

    /// 从成功值构造该 Try 类型。
    fn from_output(output: Self::Output) -> Self;

    /// 将自身分支为“继续”或“中断”。
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
}
```

同时，`?` 运算符的传播还依赖另一个 trait `FromResidual`：

```rust
pub trait FromResidual<R = <Self as Try>::Residual> {
    /// 从残差值构造该 Try 类型（通常是错误状态）。
    fn from_residual(residual: R) -> Self;
}
```

各组件含义

组件	含义
`Output`	`?` 成功传播时“剥壳”后得到的值类型。例如 `Result<T, E>` 的 `Output` 是 `T`，`Option<T>` 的 `Output` 是 `T`。
`Residual`	表示“中断/错误”状态的值类型。通常是一个不完整的版本，如 `Result<!, E>` 或 `Option<!>`。引入 `Residual` 可以避免在传播错误时携带完整类型信息，从而允许跨不同类型的错误转换。
`from_output`	从普通成功值构造该类型。例如 `Result::Ok`。
`branch`	将值分为两种可能：继续（包含 `Output`）还是中断（包含 `Residual`）。`ControlFlow` 是一个标准枚举：`ControlFlow::Continue(Output)` 和 `ControlFlow::Break(Residual)`。
`FromResidual::from_residual`	从 `Residual` 生成最终返回的错误状态。这个 trait 允许不同 `Try` 类型之间的残差转换（例如 `Result<T, E>` 和 `Result<U, E>`）。

---

# 3. `?` 运算符的脱糖过程

在实现了 `Try` + `FromResidual` 后，`?` 的行为等价于：

```rust
match Try::branch(expression) {
    ControlFlow::Continue(val) => val,
    ControlFlow::Break(residual) => return FromResidual::from_residual(residual),
}
```

这比 v1 的设计（直接要求 `into_result` + `from_error`）更通用，因为：

- 不再要求“错误类型”必须能直接转换为当前函数的返回类型；

- 允许残差类型携带额外信息（例如 Result 的残差可以只包含错误部分，而不用管 Ok 类型）；

- 通过 `FromResidual` 提供了自然的错误类型转换点。

---

# 4. 标准库的典型实现

## 4.1 `Result<T, E>` 的实现

```rust
impl<T, E> Try for Result<T, E> {
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: T) -> Self {
        Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            Ok(val) => ControlFlow::Continue(val),
            Err(e)  => ControlFlow::Break(Err(e)),
        }
    }
}

impl<T, E, F: From<E>> FromResidual<Result<Infallible, E>> for Result<T, F> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Err(e) => Err(From::from(e)),
        }
    }
}
```

**关键点**：`Residual` 被定义为 `Result<Infallible, E>`，即只有错误分支是“有意义”的，成功分支用 `Infallible` 占位，表示该残差永远是中断状态。这避免了在传播错误时还要提供 `T` 类型的信息，从而让 `?` 可以轻松地在 `Result<T, E>` 到 `Result<U, F>` 之间转换错误（只要 `E: Into<F>`）。

## 4.2 `Option<T>` 的实现

```rust
impl<T> Try for Option<T> {
    type Output = T;
    type Residual = Option<Infallible>;

    fn from_output(output: T) -> Self {
        Some(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            Some(val) => ControlFlow::Continue(val),
            None      => ControlFlow::Break(None),
        }
    }
}

impl<T> FromResidual for Option<T> {
    fn from_residual(_: Option<Infallible>) -> Self {
        None
    }
}
```

## 4.3 `Poll<Result<T, E>>` 等异步类型的实现

`Poll<Result<T, E>>` 也通过类似的模式支持 `?`，让异步代码中的错误传播同样自然。

```rust
impl<T, E> ops::Try for Poll<Result<T, E>> {
    type Output = Poll<T>;
    type Residual = Result<convert::Infallible, E>;

    #[inline]
    fn from_output(c: Self::Output) -> Self {
        c.map(Ok)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Poll::Ready(Ok(x)) => ControlFlow::Continue(Poll::Ready(x)),
            Poll::Ready(Err(e)) => ControlFlow::Break(Err(e)),
            Poll::Pending => ControlFlow::Continue(Poll::Pending),
        }
    }
}

impl<T, E, F: From<E>> ops::FromResidual<Result<convert::Infallible, E>> for Poll<Result<T, F>> {
    #[inline]
    fn from_residual(x: Result<convert::Infallible, E>) -> Self {
        match x {
            Err(e) => Poll::Ready(Err(From::from(e))),
        }
    }
}
```

## 4.4 `Poll<Option<Result<T, E>>>` 等异步类型的实现

```rust
impl<T, E> ops::Try for Poll<Option<Result<T, E>>> {
    type Output = Poll<Option<T>>;
    type Residual = Result<convert::Infallible, E>;

    #[inline]
    fn from_output(c: Self::Output) -> Self {
        c.map(|x| x.map(Ok))
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Poll::Ready(Some(Ok(x))) => ControlFlow::Continue(Poll::Ready(Some(x))),
            Poll::Ready(Some(Err(e))) => ControlFlow::Break(Err(e)),
            Poll::Ready(None) => ControlFlow::Continue(Poll::Ready(None)),
            Poll::Pending => ControlFlow::Continue(Poll::Pending),
        }
    }
}

impl<T, E, F: From<E>> ops::FromResidual<Result<convert::Infallible, E>>
    for Poll<Option<Result<T, F>>>
{
    #[inline]
    fn from_residual(x: Result<convert::Infallible, E>) -> Self {
        match x {
            Err(e) => Poll::Ready(Some(Err(From::from(e)))),
        }
    }
}
```

---

# 5. 自定义类型支持 ? 的示例

假设我们要定义一个类型 `MyResult<T, E>`，它的行为与 `Result` 类似：

```rust
#![feature(try_trait_v2)]

use std::ops::{Try, FromResidual, ControlFlow};
use std::convert::Infallible;

enum MyResult<T, E> {
    MyOk(T),
    MyErr(E),
}

impl<T, E> Try for MyResult<T, E> {
    type Output = T;
    type Residual = MyResult<Infallible, E>;

    fn from_output(output: T) -> Self {
        MyResult::MyOk(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            MyResult::MyOk(val) => ControlFlow::Continue(val),
            MyResult::MyErr(e)  => ControlFlow::Break(MyResult::MyErr(e)),
        }
    }
}

impl<T, E, F: From<E>> FromResidual<MyResult<Infallible, E>> for MyResult<T, F> {
    fn from_residual(residual: MyResult<Infallible, E>) -> Self {
        match residual {
            MyResult::MyErr(e) => MyResult::MyErr(From::from(e)),
            _ => unreachable!(),
        }
    }
}
```

现在就可以在返回 `MyResult` 的函数中使用 `?` 了：

```rust
fn test() -> MyResult<i32, String> {
    let x: MyResult<i32, &str> = MyResult::MyErr("error");
    let y = x?;  // 自动传播错误，并将 &str 转换为 String
    MyResult::MyOk(y + 1)
}
```

---

# 6. 设计演进：从 v1 到 v2

## 6.1 早期的 `Try` trait（v1）

Rust 最初（1.13 左右）引入了一个未稳定的 `Try` trait，方法为：

```rust
fn into_result(self) -> Result<Self::Ok, Self::Error>;
fn from_error(v: Self::Error) -> Self;
fn from_ok(v: Self::Ok) -> Self;
```

这个设计存在若干问题：

- 错误类型耦合：`into_result` 强制将所有类型都映射到 `Result`，限制了残差表达力；

- `Option` 的别扭实现：`Option::None` 必须模拟为 `Result::Err(())`，丢失了类型信息；

- 难以支持自定义 `Residual`：无法优雅地表达“只传播错误而不携带成功类型”的残差。

## 6.2 try_trait_v2 的引入（RFC 3058）

v2 重新设计了 trait 系统，引入：

- `ControlFlow` 枚举作为分支结果；

- `Residual` 关联类型，专门表示中断状态；

- `FromResidual` 将残差转换职责分离出来。

这一设计带来了：

- 更清晰的语义：成功与中断完全正交；

- 灵活的转换：`FromResidual` 可以按需实现，例如 `Result<T, E>` 可转换为 `Result<U, F>` 当 `E: Into<F>`；

- 与 `ControlFlow` 对齐：标准库中的 `ControlFlow` 类型本身也实现了 `Try`，使得 `?` 可用于手写的控制流。

## 6.3 当前稳定状态

截至 Rust 1.78（2024年），`Try` 和 `FromResidual` 仍为 nightly 特性（feature gate `try_trait_v2`）。稳定的 `?` 运算符目前是编译器内置实现的，并未公开 trait 给用户实现。但是，该设计已经非常成熟，标准库内部所有的 `?` 支持都基于此 trait，只是在 stable 版本中尚未开放给第三方类型。RFC 3058 已被合并，社区预计未来版本会逐步稳定这个 trait。

---

# 7. 总结

- `std::ops::Try` 是 `?` 运算符的通用抽象 trait。

- 它包含 `Output` 和 `Residual` 两个关联类型，通过 `branch` 方法将值分为“继续”或“中断”。

- 配合 `FromResidual` trait，残差可以被转换为最终返回类型的错误状态。

- 这一设计让任何类型都可以拥有类似 `Result` / `Option` 的错误传播能力，极大地扩展了 Rust 错误处理体系的灵活性。

- 虽然目前尚在 nightly，但其理念和实现方式已经稳定，学习它有助于理解 `?` 的深层机制，并为未来自定义错误传播做好准备。

掌握 `Try` trait 不仅是深入理解 Rust 控制流的关键一步，也是在库设计时提供一流 `?` 支持的必备知识。
