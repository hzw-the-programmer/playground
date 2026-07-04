# 一、syn 是什么

`syn` 是 Rust 生态最核心的语法解析库，专门用于解析 Rust 源代码字符串，生成抽象语法树（AST），是 proc-macro（过程宏）开发的底层依赖。

配套三件套：

1. syn：解析 Rust 代码 → AST 数据结构
2. quote：把 AST / 标记拼接回 Rust 代码（代码生成）
3. proc-macro2：统一 proc-macro 输入输出标记（TokenStream），抹平编译器版本差异

核心用途

- 自定义 derive 宏（`#[derive(MyMacro)]`）
- 属性宏（`#[my_attr]`）
- 函数式过程宏
- 代码静态分析、代码生成工具（序列化、ORM、路由框架底层）

# 二、基础环境配置

## Cargo.toml 依赖

```toml
[lib]
proc-macro = true  # 过程宏库必须开启

[dependencies]
# full 开启全部语法解析能力，开发宏推荐；轻量场景可按需启用 feature
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

## syn 常用 feature 精简方案（替代 full，减小编译体积）

```toml
syn = { version = "2.0", features = [
  "parsing",    # 解析代码
  "printing",   # AST 转字符串
  "derive",     # 解析 derive 相关结构体/枚举
  "structs", "enums", "unions",
  "generics",   # 泛型解析
  "attrs",      # 解析属性 #[xxx]
  "token-trees"
]}
```

# 三、核心概念分层：TokenStream → proc-macro2 → syn AST

## 1. TokenStream（标记流）

编译器传给宏的原始输入，是一堆基础词法单元：标识符、关键字、括号、字符串、数字等。

- 编译器原生：`proc_macro::TokenStream`
- 跨平台兼容包装：`proc_macro2::TokenStream`（syn/quote 统一使用）

## 2. syn 解析入口：syn::parse_macro_input!

过程宏标准模板，自动把输入 TokenStream 解析成 syn AST 类型，解析失败直接返回编译错误，无需手动错误处理。

```rust
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // 解析输入结构体/枚举的完整定义
    let ast: DeriveInput = syn::parse_macro_input!(input);
    // ... 处理 ast
}
```

## 3. syn 顶层 AST 核心类型

类型	作用	使用场景
`syn::DeriveInput`	`#[derive]` 宏专用，代表带属性、泛型、字段的结构体 / 枚举 / 联合体	derive 宏 90% 场景
`syn::Item`	任意顶层 Rust 条目：struct/enum/fn/const/trait/impl 等	属性宏、函数宏，解析任意代码块
`syn::Block`	`{}` 包裹的代码块（函数体、if 块）	解析函数内部逻辑
`syn::Expr`	任意表达式（1+1、函数调用、match、闭包）	解析表达式参数
`syn::Type`	类型：`u32`、`Vec<T>`、`&str`、`impl Trait`	提取字段类型、泛型约束
`syn::Attribute`	属性 `#[xxx(yyy)]`	读取自定义宏标记参数

# 四、DeriveInput 深度拆解（最常用）

`DeriveInput` 结构体完整结构：

```rust
struct DeriveInput {
    pub attrs: Vec<Attribute>,    // 所有 #[xxx] 属性
    pub vis: Visibility,          // 可见性 pub / 私有
    pub ident: Ident,              // 名称（结构体名/枚举名）
    pub generics: Generics,        // 泛型 <T: Debug>
    pub data: Data,                // 内部数据：结构体/枚举/联合体
}
```

## 1. data 分支 Data 枚举

```rust
enum Data {
    Struct(DataStruct),   // struct S { a: u32 } / struct S(u32) / struct S;
    Enum(DataEnum),       // enum E { A, B(u8) }
    Union(DataUnion),     // union U { x: i32 }
}
```

### 示例 1：解析结构体字段 DataStruct

```rust
let ast: DeriveInput = syn::parse_macro_input!(input);
let struct_name = &ast.ident;

match &ast.data {
    syn::Data::Struct(s) => {
        // s.fields：结构体字段集合
        match &s.fields {
            syn::Fields::Named(fields) => {
                // 命名字段 struct S { a: u32 }
                for field in &fields.named {
                    let field_name = &field.ident;
                    let field_ty = &field.ty; // 字段类型
                    let field_attrs = &field.attrs; // 字段上的 #[xxx]
                }
            }
            syn::Fields::Unnamed(fields) => {
                // 元组结构体 struct S(u32, String)
                for field in &fields.unnamed {
                    let field_ty = &field.ty;
                }
            }
            syn::Fields::Unit => {
                // 单元结构体 struct S;
            }
        }
    }
    syn::Data::Enum(e) => {
        // 遍历枚举变体
        for variant in &e.variants {
            let var_name = &variant.ident;
            let var_fields = &variant.fields;
        }
    }
    syn::Data::Union(_) => unimplemented!(),
}
```

## 2. 泛型 Generics 解析

```rust
// 提取泛型参数 <T, U: Clone>
for param in &ast.generics.params {
    match param {
        syn::GenericParam::Type(t) => {
            let ty_param = &t.ident; // T、U
            // t.bounds：trait 约束 Clone + Debug
        }
        syn::GenericParam::Lifetime(l) => {
            // 生命周期 'a
        }
        syn::GenericParam::Const(c) => {
            // const N: usize
        }
    }
}

// where 子句 ast.generics.where_clause
```

## 3. Attribute 属性解析（提取宏参数）

场景：#[my_macro(skip, rename = "abc")]

### 方式 1：简单匹配标记

```rust
let mut rename = None;
for attr in &ast.attrs {
    // 判断属性名是否 my_macro
    if attr.path().is_ident("my_macro") {
        // 解析括号内参数
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                // 读取 = "abc" 字符串值
                let lit: syn::LitStr = meta.value()?.parse()?;
                rename = Some(lit.value());
            }
            Ok(())
        })?;
    }
}
```

### 方式 2：专用结构体 + Parse 派生（复杂参数推荐）

```rust
use syn::Parse;

#[derive(Debug)]
struct MyAttrArgs {
    skip: bool,
    rename: Option<String>,
}

impl Parse for MyAttrArgs {
    fn parse(input: syn::ParseStream) -> syn::Result<Self> {
        let mut skip = false;
        let mut rename = None;

        loop {
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().as_str() {
                "skip" => skip = true,
                "rename" => {
                    input.parse::<syn::Token![=]>()?;
                    let s: syn::LitStr = input.parse()?;
                    rename = Some(s.value());
                }
                _ => return Err(input.error("未知参数")),
            }
            // 逗号分隔
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            } else {
                break;
            }
        }
        Ok(MyAttrArgs { skip, rename })
    }
}

// 使用
let args: MyAttrArgs = attr.parse_args()?;
```

# 五、手动解析：syn::parse / ParseStream

除了 `parse_macro_input!`，syn 支持手动解析任意字符串 / TokenStream。

## 1. 从字符串解析代码

```rust
let code = "struct Test { a: u32 }";
let ast: syn::Item = syn::parse_str(code)?;
```

## 2. `ParseStream` 底层流式解析原理

`ParseStream` 是 syn 的核心解析器，提供流式词法读取 API：

- `input.parse::<T>()`：读取一个语法单元，失败返回编译错误
- `input.peek::<T>()`：预读下一个标记，不消耗
- `input.is_empty()`：是否读完所有标记
- `input.error(msg)`：生成编译报错

示例：手动解析 `(u32, String)` 元组类型

```rust
fn parse_tuple_ty(input: syn::ParseStream) -> syn::Result<syn::TypeTuple> {
    let content;
    syn::parenthesized!(content in input); // 捕获 () 内部流
    let mut elems = Vec::new();
    while !content.is_empty() {
        elems.push(content.parse()?);
        if content.peek(syn::Token![,]) {
            content.parse::<syn::Token![,]>()?;
        }
    }
    Ok(syn::TypeTuple { elems })
}
```

# 六、AST 转代码：配合 quote

syn 负责解析输入 AST，quote 基于 syn 类型生成新代码 TokenStream，二者无缝配合。

```rust
use quote::quote;

let struct_name = &ast.ident;
let gen = quote! {
    impl MyTrait for #struct_name {
        fn hello() -> &'static str {
            "hello syn"
        }
    }
};

// 转换为编译器需要的 TokenStream
gen.into()
```

插值规则

- `#ident`：插入标识符、syn AST 类型（Type、Field、Generics 全部支持）
- `#(#fields),*`：循环展开（遍历结构体字段批量生成代码）

# 七、错误处理体系

syn 所有解析函数返回 `syn::Result<T>`，错误类型 `syn::Error`：

1. 自动报错：`parse_macro_input!` 捕获错误，直接返回给编译器，无需手动处理
2. 手动抛错：`input.error("参数格式错误")`，会携带代码位置，IDE 精准标红
3. 错误合并：多个错误可用 `syn::Error::combine` 合并一次性输出

```rust
if field.ty != syn::parse_str::<syn::Type>("u32")? {
    return Err(syn::Error::new_spanned(
        &field.ty,
        "字段类型必须为 u32"
    ));
}
```

`new_spanned`：绑定 AST 节点的源码跨度，编译器高亮对应代码位置。

# 八、高级特性

## 1. 遍历 AST：Visit / VisitMut

syn 提供访问者模式，递归遍历整个 AST，无需手动递归匹配每一层结构。

```rust
use syn::visit::Visit;

struct TypeCollector(Vec<syn::Type>);

impl<'a> Visit<'a> for TypeCollector {
    // 访问所有字段类型
    fn visit_type(&mut self, ty: &'a syn::Type) {
        self.0.push(ty.clone());
        syn::visit::visit_type(self, ty); // 递归子节点
    }
}

// 使用
let mut collector = TypeCollector(Vec::new());
collector.visit_derive_input(&ast);
let all_types = collector.0;
```

- `Visit`：只读遍历
- `VisitMut`：可变遍历，可修改 AST 节点

## 2. 打印 AST（调试）

开启 `printing` feature 后，将 AST 转为可读字符串调试：

```rust
println!("{:#?}", ast); // 结构化 Debug 打印
let ast_str = quote!(#ast).to_string(); // 还原源码字符串
```

## 3. 标记跨度 Span

每个 syn 节点都带 .span()，记录源码行列位置，用于精准报错、代码定位：

```rust
let span = struct_name.span();
syn::Error::new(span, "结构体名称非法");
```

# 九、完整最小可运行 Derive 宏示例

## lib.rs（过程宏库）

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(PrintFields)]
pub fn derive_print_fields(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name = &ast.ident;

    // 收集所有命名字段
    let fields = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => unimplemented!("仅支持命名字段结构体"),
        },
        _ => unimplemented!("仅支持结构体"),
    };

    let field_idents = fields.iter().map(|f| &f.ident);

    let gen = quote! {
        impl #name {
            pub fn print(&self) {
                println!("struct {}", stringify!(#name));
                #(
                    println!("{}: {:?}", stringify!(#field_idents), self.#field_idents);
                )*
            }
        }
    };

    gen.into()
}
```

## 使用端 main.rs

```rust
use my_macro::PrintFields;

#[derive(PrintFields, Debug)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let u = User { id: 1, name: "syn".into() };
    u.print();
}
```

# 十、常见踩坑点

1. feature 缺失：忘记开启 `attrs`/`generics`，解析对应结构直接编译报错
2. TokenStream 类型混淆：`proc_macro::TokenStream` 和 `proc_macro2::TokenStream` 需要互相 `.into()` 转换
3. 泛型 where 子句生成：quote 插值 `#generics` 会自动处理 `<T>` 和 where 约束
4. 未处理元组 / 单元结构体：只匹配命名字段导致 panic
5. 报错不带 span：使用 `new_spanned` 否则编译器报错无代码高亮
6. 循环插值逗号 `#(#x),*` 遗漏逗号，生成语法错误代码

# 十一、生态拓展

- `synstructure`：封装枚举变体匹配逻辑，简化 enum 宏开发
- `darling`：基于 syn 封装属性参数解析，替代手写 Parse trait，大幅简化 `#[attr(args)]` 读取
- `prettyplease`：格式化 quote 生成的代码，输出美观可读源码
