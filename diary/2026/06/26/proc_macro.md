我们来详细拆解 Rust 的三大类过程宏（Procedural Macros）。它们本质上是编译时执行的函数，接收 Rust 代码（TokenStream），对代码进行解析、转换或生成，再输出新的 TokenStream。

关键前提：定义过程宏的 crate 必须在 `Cargo.toml` 中声明 `proc-macro = true`，且只能用于 lib 目标，不能与普通业务逻辑混在同一个 crate 里。

# 1. proc_macro（函数宏）

**特征**：使用时像函数调用，带感叹号：`my_macro!(...)`。

**签名**：`fn (input: TokenStream) -> TokenStream`

**核心用途**：

接收括号内的内容作为输入，输出任意新的代码。它不附加到任何结构体或函数上，而是独立调用。

**典型场景**：

- 解析字符串语法（如 `sql!("SELECT * FROM users")`，编译时校验 SQL）。

- 实现 `format!`、`vec!` 这类构建数据结构的语法糖。

- 编译时计算或常量替换。

**代码示意**：

```rust
// 定义（在 proc_macro crate 中）
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> i32 { 42 }".parse().unwrap()
}

// 使用（在业务 crate 中）
make_answer!();
assert_eq!(answer(), 42);
```

**注意**：输入 TokenStream 仅仅是宏括号内写的内容，不包含外部上下文。

# 2. `proc_macro_derive`（派生宏）

**特征**：用于为结构体（Struct）或枚举（Enum）自动实现 `trait`，即 `#[derive(MyTrait)]`。

**签名**：`fn (input: TokenStream) -> TokenStream`

**核心用途**：

输入必须是被修饰的那个结构体/枚举的完整定义（包含字段、泛型等）。输出通常是该类型实现某 trait 的代码块（`impl ... for ...`）。

**典型场景**：

- 框架必备，如 `#[derive(Serialize, Deserialize, Debug, Clone)]`。

- 配合 `syn` 库解析 Rust 语法，针对不同字段做代码生成。

**代码示意**：

```rust
// 定义（Cargo.toml 需要引入 syn 和 quote）
#[proc_macro_derive(Hello)]
pub fn hello_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); // 解析为 DeriveInput
    let name = &ast.ident;
    quote::quote! {
        impl Hello for #name {
            fn hello() { println!("Hello from {}", stringify!(#name)); }
        }
    }.into()
}

// 使用
#[derive(Hello)]
struct MyStruct; // 自动生成 impl Hello for MyStruct
```

**重点**：派生宏只能用在结构体/枚举上，且必须输出 impl 代码块。

# 3. `proc_macro_attribute`（属性宏）

**特征**：使用时放在函数、结构体、模块、trait 等定义上方，如 `#[my_attr(arg1, arg2)]`。

**签名**：`fn (args: TokenStream, input: TokenStream) -> TokenStream`

**核心用途**：

它可以接收两个参数：

- `args`：属性括号里的参数（如 `#[route(GET, path="/")]` 里的 `GET, path="/"`）。

- `input`：**被修饰的那个项（Item）** 的完整代码（如整个函数体、整个结构体定义）。

**关键能力**：属性宏既可以保留原代码（原样输出），也可以删除、修改或包裹原代码（如添加日志、性能计时、权限校验）。它拥有对 AST 最高的控制权。

**典型场景**：

- Web 框架路由（`#[get("/")]`）。

- 测试框架（`#[test]` 本质是个属性宏，默默把函数注册到测试池）。

- AOP（面向切面编程），如自动加 `tracing` 日志。

**代码示意**：

```rust
// 定义：一个简单的计时器属性宏
#[proc_macro_attribute]
pub fn time_it(args: TokenStream, input: TokenStream) -> TokenStream {
    // 这里 args 没用到，input 是被修饰的函数
    let func = syn::parse::<syn::ItemFn>(input).unwrap();
    let name = &func.sig.ident;
    let block = &func.block;
    quote::quote! {
        fn #name() {
            let start = std::time::Instant::now();
            #block
            println!("执行耗时: {:?}", start.elapsed());
        }
    }.into()
}

// 使用
#[time_it]
fn heavy_task() { std::thread::sleep(std::time::Duration::from_secs(1)); }
```

# 核心区别与选型对比表


维度	`proc_macro`（函数式）	`proc_macro_derive`（派生）	`proc_macro_attribute`（属性）
调用语法	`mac!(...)`	`#[derive(Mac)]` 置于结构体/枚举	`#[mac(...)]` 置于任意项
输入内容	仅括号内的代码	整个结构体/枚举定义	属性参数 + 被修饰的整个项（函数、模块等）
输出内容	任意新代码（可完全不依赖输入）	必须输出 `impl` 代码块	输出新的项定义（可修改原逻辑）
是否保留原代码	无原代码概念	保留原类型定义，仅追加 `impl`	可选择性保留、删除或包裹原代码
最经典依赖库	`syn` 解析自定义语法	`syn`（解析 `DeriveInput`）+ `quote`	`syn`（解析 `ItemFn`/`ItemStruct` 等）+ `quote`
常见范例	`format!`, `const_str!`	`serde::Serialize`, `Debug`	`tokio::main`, `actix_web::get`

# 必知必会的生态库（辅助开发）

- `syn`：将 `TokenStream` 解析成可遍历的 Rust 语法树（如 `DeriveInput`、`ItemFn`）。

- `quote`：利用 `quote::quote! { ... }` 宏，将 Rust 代码模板转为 `TokenStream`，支持 `#var` 插值。

- `proc_macro2`：提供更强大的 `TokenStream` 操作，且能在稳定版 Rust 上模拟 nightly 特性。

# 重要限制（稳定性）

1. 过程宏必须是卫生的（Hygienic），即生成的标识符默认不会污染外部作用域（除非显式使用 #ident）。

2. 定义宏的 crate 导出项必须标注 `#[proc_macro]` 等属性，且不能与普通 `pub fn` 混用。

3. 编译时，过程宏会先被编译并执行，再编译依赖它的业务代码，因此会拖慢编译速度，但带来了极强的运行时零开销抽象。
