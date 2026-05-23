`#[track_caller]` 是 Rust 的一个函数属性，它的核心作用是：当函数内部发生 `panic` 时，`panic` 的定位信息（文件、行号）会指向调用该函数的地方，而不是函数内部真正 `panic` 的代码。

我们结合这个 `todo` 函数来详解：

```rust
#[track_caller]
pub fn todo<Input, Output, Error>(input: &mut Input) -> Result<Output, Error>
where
    Input: Stream,
    Error: ParserError<Input>,
{
    #![allow(clippy::todo)]
    trace("todo", move |_input: &mut Input| {
        todo!("unimplemented parse")
    })
    .parse_next(input)
}
```

1. 这个函数是做什么的？

这是解析库（如 `winnow`）提供的一个占位解析器。它什么都不解析，一旦被调用就立即 `panic`，提示“这里还没实现”。用法类似标准库的 `todo!()` 宏，但它是作为解析器组合子存在的，可以嵌入在解析器组合链条中：

```rust
seq!(MyStruct {
    field: parser1,
    other: todo,  // 暂时还没想好怎么解析 other 字段
})
```

2. 没有 `#[track_caller]` 会发生什么？

如果不加这个属性，当 `todo` 被调用时，`panic!` 发生在闭包内部，而闭包又是在 `trace` 组合子内部被调用的。Rust 的 `panic` 回溯会显示：

```text
panicked at 'unimplemented parse', src/combinator/trace.rs:42:14
```

或者更糟糕，因为函数体有一系列间接调用（`trace(...).parse_next(input)`），最终报告的位置很可能是这个 `todo` 函数本身的定义行（比如 `src/combinator/todo.rs:10`），而不是你写下 `todo` 占位的那一行。

这对开发者极不友好——你明明是在 `parser.rs:123` 处写了 `other: todo`，报错却指向库的内部代码，根本找不到是自己哪里用了未实现的解析器。

3. 加上 `#[track_caller]` 之后

该属性会让编译器做两件事：

- 自动将调用者的位置（call site）作为隐式参数传递给函数。

- 函数内所有带 `#[track_caller]` 的 `panic` 相关操作（如 `panic!`、`unwrap`、`expect`，以及宏 `todo!` 实际展开的 `panic!`）都会使用这个位置信息，而不是自身的位置。

因此，当 `todo` 解析器最终触发 `todo!("unimplemented parse")` panic 时，错误消息会直接定位到：

```text
panicked at 'unimplemented parse', src/my_parser.rs:123:12
```

也就是你写出 `other: todo` 的那个位置。这使得开发者可以一眼看出是哪个解析器还没有实现，无需深入库的调用栈。

4. 工作原理细节

- todo! 宏内部是 panic!("not yet implemented")（或类似消息），而 panic! 本身是带有 #[track_caller] 的。因此只要 todo! 所在的函数拥有调用者位置信息，它就会传递下去。

- trace("todo", move |_input| { ... }) 会创建一个解析器，并在其 parse_next 中调用闭包。闭包并不是 #[track_caller] 的函数（闭包默认继承调用者的 #[track_caller] 属性吗？实际上，闭包会捕获调用者位置信息，如果闭包直接 panic，位置会正确；但如果隔了多层，属性必须出现在包含 panic 的函数上）。这里的 #[track_caller] 放在最外层 fn todo 上，就保证了从用户调用 todo 到内部 panic! 的整个调用链中，用户调用点信息不会丢失。

- 函数体中的 #![allow(clippy::todo)] 是一个内部属性，用于抑制 clippy 对 todo! 宏的警告，表明这里故意使用 todo!。

5. 总结

#[track_caller] 在这个场景下是一个错误定位优化，它让“未实现的解析器”在 panic 时指向开发者写出该占位符的位置，而不是解析库内部的实现位置。这在调试大型解析器时非常有价值，因为你可以立刻知道是哪个组合子还没有替换成真正的解析逻辑。

```rust
// 某业务解析代码
fn my_parser(input: &mut &str) -> PResult<MyStruct> {
    seq!(MyStruct {
        name: alpha1,
        // 假设这里还没实现
        value: todo
    })
    .parse_next(input)
}
// 如果 value: todo 被调用，panic 信息会指向 "value: todo" 这一行。
```

