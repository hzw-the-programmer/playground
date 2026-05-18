`macro_rules!` 是 Rust 的**声明宏**（declarative macros）系统。它允许你通过**模式匹配**和**语法模板**来编写**自定义的语法扩展和代码生成规则**，是编译期元编程的核心工具之一。

下面是详细拆解：

---

1. **`macro_rules!` 的本质**

它是一种**以“匹配 -> 替换”为工作方式**的宏，在编译的早期阶段（语法分析后）对令牌流（`TokenStream`）进行操作。你可以把它看作一个代码变换的“查找替换”，但远比 C 的宏强大，并且具备一定的卫生性（hygiene）。

Rust 中宏分为两大类：

- **声明宏**（`macro_rules!`，也称“宏 2.0”之前的旧称）

- **过程宏**（`#[proc_macro]` 等，可以对 `TokenStream` 进行任意操作）

本回答聚焦声明宏。

---

2. **基本结构**

```rust
macro_rules! 宏名 {
    (模式1) => { 展开1 };
    (模式2) => { 展开2 };
    // ...
}
```

- **宏名**：调用时使用 **宏名!()**。

- **每条规则**由**模式**和**展开体**组成，分号分隔。

- 模式匹配**调用时传入的令牌序列**，展开体生成代码。

最简单的例子：

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

say_hello!(); // 调用
```

---

3. **捕获与片段说明符（Fragment Specifiers）**

宏通过 $变量名:片段类型 捕获输入。片段类型决定了可以匹配的语法类别。

常用片段类型：

片段说明符	匹配内容
ident	标识符（变量名、函数名等）
ty	类型（如 i32, Vec<u8>）
expr	表达式
block	花括号包裹的块（{ ... }）
stmt	单个语句（不含末尾分号）
pat	模式（Some(x) 等）
path	路径（std::collections::HashMap）
meta	元项（#[...] 里的内容）
item	项（函数、结构体定义等）
vis	可见性（pub, pub(crate) 等）
lifetime	生命周期（'a）
literal	字面量（42, "hello", true）
tt	令牌树（TokenTree），几乎可匹配任何内容

示例：

```rust
macro_rules! create_function {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("You called {:?}()", stringify!($fn_name));
        }
    };
}

create_function!(foo);
foo(); // 打印：You called "foo"()
```

---

4. **重复（Repetition）**

使用 `$(...)*` 或 `$(...)+` 捕获重复的令牌序列，并用 `$(...)*` 在展开中重复。

语法：

- `$()*` —— 零次或多次

- `$()+` —— 一次或多次

可带分隔符：`$(),*` 表示逗号分隔，零次或多次；`$();+` 表示分号分隔，至少一次。

```rust
macro_rules! vec_strs {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

let v = vec_strs!["a", "b", "c"]; // 展开为 vec!["a".to_string(), "b".to_string(), "c".to_string()]
```

重复中还可以嵌套：

```rust
macro_rules! print_pairs {
    ($($key:expr => $value:expr),*) => {
        $(println!("{}: {}", $key, $value);)*
    };
}
```

---

5. **规则匹配与顺序**

宏规则按**从上到下**的顺序尝试匹配。一旦匹配成功，就使用该规则的展开体，不会尝试后续规则。

因此，更具体、更严格的模式应放在前面，宽泛的模式放在后面。

```rust
macro_rules! number {
    (one) => { 1 };
    ($x:expr) => { $x };
}
number!(one);  // 匹配第一条规则，得到 1
number!(42);   // 匹配第二条规则，得到 42
```

---

6. **卫生性（Hygiene）**

Rust 宏具有**部分卫生性**，防止宏内部定义的变量与调用处环境冲突，同时也不会污染外部作用域。但并非完全卫生。

6.1 定义遮蔽

宏内部定义的 let 变量不会与外部同名变量冲突（每个声明都有自己的语法上下文）。

```rust
macro_rules! foo {
    () => {
        let x = 1;
        println!("{}", x);
    };
}
let x = 42;
foo!(); // 打印 1，外部的 x 不受影响
```

6.2 卫生的局限：标识符传递

如果你把外部捕获的 ident 直接用于定义变量或名称，那么它使用的是调用处的上下文，这可能导致预期外的行为。例如，宏可以“创造”一个调用处可见的变量，这被称为“名称导入”。

```rust
macro_rules! make_var {
    ($name:ident) => { let $name = 10; };
}
make_var!(secret);
println!("{}", secret); // 可以访问，因为 $name 是在调用处解析
```

因此在 `if_downcast_into` 宏中，我们故意用 `let $val = ...` 重新绑定传入的标识符，从而在 `$body` 内部以新的类型使用同一个名称，这属于卫生机制为我们服务的例子。

6.3 如何处理变量冲突

由于 `macro_rules!` 无法自动为内部辅助变量生成唯一名称（像过程宏可以生成新 Ident），你需要自己小心。常见的做法：

- 使用晦涩的名称或约定前缀/后缀，降低冲突概率。

- 引入新的作用域块 `{ ... }` 限制变量作用域。

- 如果必须在多次展开中保持同一变量，考虑使用闭包或其他技巧。

之前宏中的 `let mut slot = ...` 就是一个潜在冲突点——如果外部恰好也有 `slot` 变量，尽管卫生性会使得它不同于外部的 `slot`，但在某些版本和场景下存在隐患。一个更健壮的做法是将逻辑包裹在闭包里，或者干脆用不常见的名字。

---

7. **高级模式：递归宏与 TT 咀嚼**

7.1 递归宏

声明宏可以调用自身（需注意递归深度）。

```rust
macro_rules! sum {
    ($e:expr) => { $e };
    ($e:expr, $($rest:expr),*) => {
        $e + sum!($($rest),*)
    };
}
sum!(1, 2, 3); // 展开为 1 + (2 + (3))
```

7.2 TT 咀嚼（TT munching）

利用 tt 片段和递归，可以处理复杂的括号结构，甚至构建 DSL。

```rust
macro_rules! tiny_map {
    // 基本情况：空列表
    ({}) => { ::std::collections::HashMap::new() };
    // 处理一个键值对
    ({ $($key:expr => $value:expr),* $(,)? }) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}
let m = tiny_map!({ "a" => 1, "b" => 2 });
```

---

8. **内用规则与 @ 模式**

可以在宏内部定义辅助宏（用 macro_rules!），或者使用**内部规则**（internal rules）模式，即在规则前面加一个特殊标记（如 @）来标记实现细节，防止外部误调用。

```rust
macro_rules! add {
    // 公开入口
    ($x:expr, $y:expr) => { add!(@impl $x, $y) };
    // 内部实现
    (@impl $x:expr, $y:expr) => { $x + $y };
}
```

---

9. **调试与工具**

- **stringify!**：将任意令牌转为字符串。

- **log_syntax!**（nightly）：打印宏展开过程的语法树。

- **trace_macros!**：控制台展示宏调用和展开。

- **cargo expand**（需安装 cargo-expand）：查看宏展开后的完整代码。

---

10. **常见陷阱**

1. 花括号与括号不匹配：`macro_rules!` 对括号很敏感，常因重复模式中的括号导致奇怪错误。

2. 分隔符的模糊性：`$()*` 和 `$(),*` 细小的不同导致匹配失败。

3. 卫生性带来的“意外”：在 `macro_rules!` 中，你不能像过程宏那样拼接标识符（如 paste crate 的做法），只能通过捕获的 ident 直传。

4. 递归深度限制：递归宏展开过多会导致编译错误，默认限制大约是 128 次展开。

5. 导出宏：使用 `#[macro_export]` 将宏暴露给外部 crate，避免污染命名空间时可借助 `#[macro_export]` 和 `$crate`。

---

11. **与 if_downcast_into 的联系**

回头看之前的宏：

```rust
macro_rules! if_downcast_into {
    ($in_ty:ty, $out_ty:ty, $val:ident, $body:expr) => {{
        if std::any::TypeId::of::<$in_ty>() == std::any::TypeId::of::<$out_ty>() {
            let mut slot = Some($val);
            let $val = (&mut slot as &mut dyn std::any::Any)
                .downcast_mut::<Option<$out_ty>>()
                .unwrap()
                .take()
                .unwrap();
            $body
        }
    }};
}
```

它演示了多个片段说明符：`ty`, `ident`, `expr`。`$val:ident` 捕获标识符，并在展开中用 `let $val = ...` 重新绑定，让 `$body` 可以按新类型使用 `$val`。整个宏展开是一个表达式块（`{{ ... }}`），保证了 `if` 表达式在无 `else` 时返回 `()`。

---

12. **过程宏（简单对比）**

- 声明宏：纯模式匹配，学习曲线平缓，适合常规代码生成。

- 过程宏：需独立的 crate（proc-macro = true），可完全操作 `TokenStream`，可生成新标识符、修改属性、实现派生等，功能更强但更复杂。

当 `macro_rules!` 无法胜任时（比如需要操作 `token` 树、注入文件位置信息、自动生成文档等），可以考虑过程宏。

---

**总结**

`macro_rules!` 是 Rust 元编程的基础，掌握它需要理解**片段说明符**、**重复语法**和**卫生性**的运作方式。它的优势在于简洁直观，能够消除大量样板代码；劣势在于调试困难和偶尔的卫生性陷阱。你可以在几乎任何 Rust 项目中看到它的身影，从 `vec![]`、`println!` 到自己编写的领域特化语言。
