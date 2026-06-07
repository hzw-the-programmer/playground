`&str` 的 `Deserialize` 实现与 `String` 截然不同：它的核心目标是零拷贝借用，因此只接受生命周期与输入数据绑定的引用。如果数据源无法提供这种借用，反序列化会直接失败。下面从生命周期、`Visitor` 实现、与 `Deserializer` 的配合三个层面进行详解。

---

# 1. `Deserialize<'de>` 生命周期与 `&str` 的角色

`serde` 中反序列化的入口是带生命周期的 trait：

```rust
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

这里的 `'de` 代表输入数据的生命周期。

`String` 实现的是 `Deserialize<'de>`，但它不持有 `'de` 引用，因此可以脱离输入数据独立存在（属于 `DeserializeOwned`）。

`&'de str` 也实现了 `Deserialize<'de>`，但它的值直接引用输入数据，必须与 `'de` 严格绑定。这意味着只有当输入数据在整个 `&str` 使用期间都有效时，反序列化才能成功。

---

# 2. `Visitor` 实现：只接受借用数据

`&str` 的 `Deserialize` 实现同样依赖 `Visitor` 模式。但与 `String` 不同，它构造的 `Visitor` 只实现了 `visit_borrowed_str` 和 `visit_borrowed_bytes`，而不实现 `visit_str` 或 `visit_string`。

简化后的源码逻辑如下：

```rust
use serde::de::{Deserialize, Deserializer, Visitor, Error};

impl<'de: 'a, 'a> Deserialize<'de> for &'a str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BorrowedStrVisitor;

        impl<'de> Visitor<'de> for BorrowedStrVisitor {
            type Value = &'de str;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a borrowed string")
            }

            // 核心方法：直接返回输入的 &'de str，零拷贝
            fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<&'de str, E> {
                Ok(v)
            }

            // 兼容二进制格式：借用字节切片并校验 UTF-8
            fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<&'de str, E> {
                std::str::from_utf8(v).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
            }
        }

        // 提示 Deserializer：希望提供借用字符串（但不是必须）
        deserializer.deserialize_str(BorrowedStrVisitor)
    }
}
```

关键点解析：

- 没有 `visit_str`：`visit_str` 的参数是 `&str`，但其生命周期不保证是 `'de`（它可能是从临时 `String` 创建的），因此无法安全返回为 `&'de str`。

- 没有 `visit_string`：直接拿到的 `String` 也无法变为对输入数据的引用。

- 只有 `visit_borrowed_str` 和 `visit_borrowed_bytes` 能从数据源本身提取出生命周期为 `'de` 的引用。

如果 `Deserializer` 无法调用这两个方法中的任何一个，就会触发 expecting 报错，例如 "invalid type: string "abc", expected a borrowed string"。

---

# 3. 与 `Deserializer` 的交互：何时成功，何时失败

`&str` 的反序列化严重依赖 `Deserializer` 提供对原始输入数据的直接视图。来看不同场景：

## 3.1 成功路径：数据源支持零拷贝借用

以 `serde_json` 为例，如果从一个 `&[u8]` 解析，且 JSON 字符串不包含转义序列，解析器可以直接指向原始字节，此时调用链为：

- 用户调用 `serde_json::from_slice::<&str>(b"\"hello\"")`。

- `Deserializer` 解析到 JSON 字符串 `"hello"`，看到目标要求 `deserialize_str`。

- 由于它持有输入字节切片 `b"\"hello\""`，并且 `hello` 部分就是原始字节中的一段有效 UTF-8，它可以直接调用 `visitor.visit_borrowed_bytes(b"hello")`。

- `BorrowedStrVisitor` 校验 UTF-8 后返回 `Ok("hello")`，全程无拷贝。

对于纯文本格式或某些二进制格式，类似逻辑同样适用。

## 3.2 失败路径：数据源无法提供借用

极常见的情况：`serde_json::from_str::<&str>("\"hello\"")` 通常会失败。原因是 `from_str` 会把输入 `&str` 适配为 `io::Read`，解析过程中需要处理转义、内存分配等，最终生成的字符串可能不是对原始输入的引用，它只能提供 `visit_str`。此时 `BorrowedStrVisitor` 无法响应，错误信息为：
"invalid type: string "hello", expected a borrowed string"。

这也是为什么直接使用 `serde_json::from_str` 反序列化 `&str` 会报错，而使用 `Cow<str>` 则能正常工作的原因。

---

# 4. 与 `Cow<'de, str>` 的对比

`Cow<'de, str>` 是 `&str` 与 `String` 的桥梁，它的 `Visitor` 实现了：

- `visit_borrowed_str` → 返回 `Cow::Borrowed`（零拷贝）

- `visit_str` → 返回 `Cow::Owned(v.to_owned())`（必要时拷贝）

- `visit_string` → 返回 `Cow::Owned(v)`

因此 `Cow` 在任何格式下都能成功：数据源能借用则借用，否则自动回退到分配。而 `&str` 则是一种“非零拷贝不可”的极端选择，完全放弃了灵活性，换来极致性能。

---

# 5. 设计意图总结

`&str` 的 `Deserialize` 实现体现了 `serde` 对零拷贝反序列化的一等支持：

类型	能否零拷贝	失败条件	适用场景
`String`	不能（必须复制）	无（总能成功）	需要所有权
`&'de str`	必须零拷贝	数据源不提供借用时立即失败	性能敏感的临时只读访问
`Cow<'de, str>`	优先零拷贝，必要时拷贝	无（总能成功）	灵活，兼顾性能与健壮性

这种层层递进的设计，让开发者可以根据对生命周期、性能和容错的实际要求，选择最合适的类型。理解 `&str` 的实现，就能明白为何有些看似简单的反序列化会失败，以及如何通过 `Cow` 优雅地解决它。
