Rust 的 `serde` 是一个高效、通用的 序列化与反序列化框架。它的核心设计是把数据结构与具体的数据格式解耦：你的结构体只需实现 `Serialize` / `Deserialize`，就可以无缝支持 `JSON`、`YAML`、`MessagePack`、`Bincode` 等格式。下面我们从核心概念到实战技巧，系统性地拆解 `serde` 的 API。

# 1. `serde` 的数据模型

`serde` 不关心数据的最终格式，它内部定义了一套自描述的数据模型，涵盖以下类型：

- 基本类型：bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, char

- 字符串：&str、String（或字节串 &[u8]）

- 序列 (Sequence)：类似数组、Vec，长度在序列化时可能未知

- 映射 (Map)：键值对集合，类似 HashMap

- 结构体 (Struct)：有命名字段的结构

- 枚举 (Enum)：多种可能的变体，每个变体可以有不同的数据

- 元组 / 元组结构体：定长、异构序列

- 单元类型：()、None 等

任何 Rust 类型，只要能映射到这个数据模型，就能序列化；从模型映射回来，就能反序列化。这套模型使得 serde 与格式完全解耦。

# 2. 核心 trait：`Serialize` 和 `Deserialize`

## 2.1 `Serialize`

```rust
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
```

任何可序列化的类型都需要实现 `serialize`。它将自身分解成数据模型的若干基本调用，交给 `serializer` 去渲染为具体格式。`Serializer` trait 定义了与数据模型对应的系列方法（见第 4 节）。

## 2.2 `Deserialize`

```rust
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

反序列化的入口。`deserialize` 接收一个 `Deserializer`，它负责从格式中解析出数据，然后通过访问者 (Visitor) 模式把解析到的数据模型元素回传，最终构建出 Rust 类型。

## 2.3 `DeserializeOwned`

`Deserialize<'de>` 的泛型参数是生存周期，它表示反序列化后的数据可能借用输入。如果希望得到完全自有的数据（不借用），可使用 `DeserializeOwned` 作为 trait bound：

```rust
pub trait DeserializeOwned: for<'de> Deserialize<'de> {}
```

大多数场景用 `DeserializeOwned` 会更方便。

# 3. 派生宏：零样板实现

99% 的场景都可以通过 `derive` 自动实现 trait：

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    #[serde(default)]
    hobbies: Vec<String>,
}
```

这会为 `Person` 生成：序列化时按结构体字段顺序写入；反序列化时按字段名匹配。你还可以通过 容器属性 和 字段属性 精确控制行为。

# 4. `Serializer` 和 `Deserializer` trait（格式开发者视角）

这两个 trait 是 `serde` 与具体格式之间的桥梁。

## 4.1 `Serializer`

格式作者需实现：

```rust
pub trait Serializer {
    type Ok;
    type Error: Error;
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTuple: SerializeTuple<...>;
    type SerializeTupleStruct: ...;
    type SerializeTupleVariant: ...;
    type SerializeMap: SerializeMap<...>;
    type SerializeStruct: SerializeStruct<...>;
    type SerializeStructVariant: ...;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> ...;
    // ... 所有基本类型
    fn serialize_str(self, v: &str) -> ...;
    fn serialize_bytes(self, v: &[u8]) -> ...;
    fn serialize_none(self) -> ...;
    fn serialize_some<T: ?Sized>(self, value: &T) -> ...;
    fn serialize_unit(self) -> ...;
    fn serialize_unit_struct(self, name: &'static str) -> ...;
    fn serialize_unit_variant(...) -> ...;
    fn serialize_newtype_struct<T: ?Sized>(...) -> ...;
    fn serialize_newtype_variant<T: ?Sized>(...) -> ...;
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, ...>;
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, ...>;
    fn serialize_tuple_struct(...) -> ...;
    fn serialize_tuple_variant(...) -> ...;
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, ...>;
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, ...>;
    fn serialize_struct_variant(...) -> ...;
}
```

当调用一个类型的 `serialize` 时，它会根据数据模型调用这些方法。例如 `Vec<i32>` 会先调用 `serialize_seq(len)` 获得一个 `SerializeSeq` 状态机，然后对每个元素调用 `serialize_i32`，最后调用 `end()`。而结构体调用 `serialize_struct`，获得 `SerializeStruct` 后依次调用 `serialize_field`。

## 4.2 `Deserializer` 和 `Visitor`

反序列化端则利用访问者模式。`Deserializer` 同样定义了一族 `deserialize_*` 方法，每个方法接收一个 `Visitor`：

```rust
pub trait Deserializer<'de> {
    type Error: Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de>;
    fn deserialize_bool<V>(self, visitor: V) -> ...;
    fn deserialize_i8<V>(self, visitor: V) -> ...;
    // ... 其他基本类型
    fn deserialize_str<V>(self, visitor: V) -> ...;
    fn deserialize_string<V>(self, visitor: V) -> ...;
    fn deserialize_bytes<V>(self, visitor: V) -> ...;
    fn deserialize_byte_buf<V>(self, visitor: V) -> ...;
    fn deserialize_option<V>(self, visitor: V) -> ...;
    fn deserialize_unit<V>(self, visitor: V) -> ...;
    fn deserialize_unit_struct<V>(...) -> ...;
    fn deserialize_newtype_struct<V>(...) -> ...;
    fn deserialize_seq<V>(self, visitor: V) -> ...;
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> ...;
    fn deserialize_tuple_struct<V>(...) -> ...;
    fn deserialize_map<V>(self, visitor: V) -> ...;
    fn deserialize_struct<V>(...) -> ...;
    fn deserialize_enum<V>(...) -> ...;
    fn deserialize_identifier<V>(...) -> ...; // 用于结构体字段名或枚举变体名
    fn deserialize_ignored_any<V>(...) -> ...; // 跳过不需要的数据
}
```

`Visitor` trait 用于构造最终结果：

```rust
pub trait Visitor<'de>: Sized {
    type Value;
    fn expecting(&self, f: &mut Formatter) -> fmt::Result;
    fn visit_bool(self, v: bool) -> Result<Self::Value, ...> { ... }
    fn visit_i8(self, v: i8) -> ...;
    // ... 所有 visit 方法
    fn visit_str(self, v: &str) -> ...;
    fn visit_string(self, v: String) -> ...;
    fn visit_bytes(self, v: &[u8]) -> ...;
    fn visit_byte_buf(self, v: Vec<u8>) -> ...;
    fn visit_none(self) -> ...;
    fn visit_some<D>(self, deserializer: D) -> ...;
    fn visit_unit(self) -> ...;
    fn visit_newtype_struct<D>(self, deserializer: D) -> ...;
    fn visit_seq<A>(self, seq: A) -> ... where A: SeqAccess<'de>;
    fn visit_map<M>(self, map: M) -> ... where M: MapAccess<'de>;
}
```

当 `Deserialize` 实现调用 `deserializer.deserialize_struct(...)` 时，格式会将字段名与值依次喂给 `MapAccess`，而类型侧的 `Visitor` 则从中取出字段并组装。这套设计让解析 JSON、XML 甚至数据库行都有一套统一的方式。

普通用户极少需要实现这两个 trait，除非你在开发一种新的数据格式支持库。

# 5. 使用已有格式（以 `serde_json` 为例）

```rust
use serde_json;

let person = Person { name: "Alice".into(), age: 30, hobbies: vec![] };

// 序列化
let json = serde_json::to_string(&person)?;
let pretty = serde_json::to_string_pretty(&person)?;
let bytes = serde_json::to_vec(&person)?;
let writer = std::io::stdout();
serde_json::to_writer(writer, &person)?;

// 反序列化
let p: Person = serde_json::from_str(&json)?;
let p: Person = serde_json::from_slice(&bytes)?;
let reader = std::io::stdin();
let p: Person = serde_json::from_reader(reader)?;

// 泛型值 serde_json::Value
let value: serde_json::Value = serde_json::from_str(&json)?;
println!("{}", value["name"]);
```

其他格式如 `serde_yaml`, `bincode`, `toml` 的 API 高度一致，只是函数前缀不同。

# 6. 属性大全：用 `#[serde(...)]` 控制行为

`serde` 提供了丰富的属性，分为容器属性（用于结构体/枚举定义）和字段属性。

## 6.1 容器属性

属性	作用
`rename_all = "..."`	字段名自动转换：`"lowercase"`, `"UPPERCASE"`, `"PascalCase"`, `"camelCase"`, `"snake_case"`, `"SCREAMING_SNAKE_CASE"`, `"kebab-case"`, `"SCREAMING-KEBAB-CASE"`
`deny_unknown_fields`	遇到 JSON 中未定义的字段时报错（默认忽略）
`tag = "type"`	枚举的内部标记字段名（见 8.2）
`content = "value"`	邻接标记的内容字段名
`untagged`	枚举不保留变体标识，逐个尝试反序列化
`bound = "..."`	为生成的泛型 impl 块添加额外的 trait bound
`crate = "..."`	指定 `serde` 的路径（如 `serde` 或 `serde::ser`）

示例：

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Config {
    max_connections: u32,
}
// 会期望 JSON: {"maxConnections": 100}
```

## 6.2 字段属性

属性	作用
`rename = "..."`	单独重命名字段
`alias = "..."`	反序列化时增加别名（可多个：alias = "a", alias = "b"）
`default`	缺失时使用 `Default::default()`
`default = "function_path"`	缺失时调用指定函数
`skip_serializing_if = "function"`	条件跳过，如 `Option::is_none`
`skip`	跳过该字段的序列化和反序列化
`serialize_with = "path"`	自定义序列化函数
`deserialize_with = "path"`	自定义反序列化函数
`with = "module"`	同时指定序列化/反序列化模块（模块内需提供 `serialize` 和 `deserialize` 函数）
`borrow` / `borrow = "..."`	从输入数据借用（零拷贝），需要生存周期 `'de`
`flatten`	将该字段的内容“平铺”到父级结构中
`bound = "..."`	单独为字段指定 trait bound

示例：

```rust
#[derive(Serialize, Deserialize)]
struct Request {
    #[serde(rename = "userName")]
    user_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(default = "default_lang")]
    lang: String,
}
fn default_lang() -> String { "en".into() }
```

# 7. 手动实现序列化/反序列化

当派生宏无法满足需求（例如高度定制的格式、需要解密字段、版本迁移）时，可以手动实现。

## 7.1 自定义序列化

```rust
use serde::{Serialize, Serializer};

impl Serialize for MyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 利用 serializer 的 API 构建数据模型
        let mut s = serializer.serialize_struct("MyType", 2)?;
        s.serialize_field("x", &self.x)?;
        s.serialize_field("y", &self.y)?;
        s.end()
    }
}
```

## 7.2 自定义反序列化 + Visitor

```rust
use serde::{Deserialize, Deserializer, de::{self, Visitor, MapAccess}};
use std::fmt;

impl<'de> Deserialize<'de> for MyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyTypeVisitor;

        impl<'de> Visitor<'de> for MyTypeVisitor {
            type Value = MyType;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a map with fields x and y")
            }

            fn visit_map<A>(self, mut map: A) -> Result<MyType, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        _ => { map.next_value::<de::IgnoredAny>()?; } // 忽略未知字段
                    }
                }
                let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                Ok(MyType { x, y })
            }
        }

        deserializer.deserialize_struct("MyType", &["x", "y"], MyTypeVisitor)
    }
}
```

使用 `deserialize_with` 简化

更常见的轻量定制是写一个函数，然后通过字段属性指定：

```rust
fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where D: Deserializer<'de> {
    let s: String = String::deserialize(deserializer)?;
    Ok(s.split(',').map(ToString::to_string).collect())
}

#[derive(Deserialize)]
struct Filter {
    #[serde(deserialize_with = "deserialize_comma_separated")]
    tags: Vec<String>,
}
```

# 8. 枚举的表示形式

枚举在序列化时有多种表现方式，通过容器属性切换。

## 8.1 外部标记（默认）

```rust
#[derive(Serialize, Deserialize)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
// JSON: {"Quit": null}
//       {"Move": {"x": 1, "y": 2}}
//       {"Write": "hello"}
```

## 8.2 内部标记 (Internally Tagged)

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
// JSON: {"type": "Quit"}
//       {"type": "Move", "x": 1, "y": 2}
//       {"type": "Write", "content": "hello"}  // 包含数据的变体默认字段名是 "content"，可通过 #[serde(content = "value")] 修改
```

## 8.3 邻接标记 (Adjacently Tagged)

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum Message { ... }
// JSON: {"type": "Quit", "value": null}
//       {"type": "Move", "value": {"x": 1, "y": 2}}
```

## 8.4 无标记 (Untagged)

```rust
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Message { ... }
// 直接尝试按变体顺序反序列化：尝试 Quit(null)，失败则试 Move{...}，再失败则试 Write(String)
```

无标记枚举序列化时直接写出数据，不包含变体标识，因此通常用于 `serde_json::Value` 这样的万能类型。

# 9. 扁平化 (flatten)

`flatten` 可以将一个子结构体的字段提升到父级，对组合和结构化数据十分有用：

```rust
#[derive(Serialize, Deserialize)]
struct Pagination {
    limit: u32,
    offset: u32,
}

#[derive(Serialize, Deserialize)]
struct Query {
    keyword: String,
    #[serde(flatten)]
    pagination: Pagination,
}
// JSON: {"keyword": "rust", "limit": 10, "offset": 0}
```

反序列化时，未知字段会自动尝试匹配 `flatten` 的子结构，所以一个结构体只能有一个 `flatten` 字段，且它必须是最后一个字段。

# 10. 零拷贝反序列化与借用

当反序列化的数据生存期足够长时，可以直接借用原始字节，避免内存分配。

```rust
#[derive(Deserialize)]
struct Data<'de> {
    #[serde(borrow)]
    name: &'de str,
    #[serde(borrow)]
    tags: Vec<&'de str>,
}

let json = r#"{"name": "Alice", "tags": ["a", "b"]}"#;
let data: Data = serde_json::from_str(json)?;
// data.name 直接指向原字符串的内存
```

对于 Cow 类型，你还可以用 `#[serde(borrow)]` 实现“有则借用，无则分配”。

# 11. 错误处理

`serde` 的错误类型是 `serde::de::Error` 和 `serde::ser::Error`，自定义实现中可通过以下关联函数构造错误：

- `Error::custom(msg)` – 通用错误

- `Error::missing_field(field)` – 缺失字段

- `Error::unknown_field(field, expected)` – 未知字段

- `Error::invalid_value(unexp, &exp)` – 值不匹配

- `Error::invalid_length(len, &expected)` – 长度错误

例如：

```rust
fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
where A: MapAccess<'de> {
    ...
    Err(Error::missing_field("name"))
}
```

# 12. 泛型、生命周期与 trait bound

当你的类型是泛型时，`derive` 会自动添加 `T: Serialize` / `T: Deserialize<'de>` 约束。如果想放宽限制，可使用 `bound` 属性：

```rust
#[derive(Serialize, Deserialize)]
#[serde(bound = "T: MyCustomTrait")]
struct Wrapper<T> {
    value: T,
}
```

字段级别同样支持：

```rust
#[serde(bound(deserialize = "T: MyDeserTrait"))]
value: T,
```

对于包含引用的类型，生命周期 `'de` 会在 `Deserialize` 实现中出现：

```rust
#[derive(Deserialize)]
struct RefWrapper<'de> {
    #[serde(borrow)]
    s: &'de str,
}
```

# 总结

`serde` 通过数据模型抽象和 trait 分离，把序列化逻辑与格式完全解耦。日常使用你只需掌握：

- 派生宏 + 属性 完成 90% 的工作。

- 了解 枚举标记形式 和 扁平化，设计灵活的数据表示。

- 必要时通过 `deserialize_with` / `serialize_with` 或 手动实现 `Visitor` 进行深度定制。

它的设计哲学是：数据如何表示由格式决定，数据与格式之间的映射由 `serde` 数据模型和 trait 实现完成。这样的分层使得 Rust 拥有了一个极其健壮、可扩展且零成本的序列化生态。
