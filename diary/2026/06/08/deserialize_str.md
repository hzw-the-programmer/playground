`deserialize_str` 的作用是告诉反序列化器：“接下来的数据应当被解析为一个 字符串，请通过 `visitor` 把值交给我。”

根据 `Serde` 的数据模型，字符串是一等公民，与字节序列（`[u8]`）是两种不同的类型。因此 一个严格遵循协议的实现，通常只会调用字符串相关的 `visit` 方法，而字节相关方法则留给 `deserialize_bytes`。

下面按你列出的方法逐一说明，在 `deserialize_str` 中它们 分别什么时候会被调用。

---

# 1. `visit_borrowed_str(&'de str)`

调用时机：反序列化器可以直接从输入数据中借用一个 `&str`，且这个引用的生命周期与输入数据 `'de` 绑定。

- 典型场景：从 `&str` 或 `&[u8]` 输入反序列化 JSON 时，如果解析到的是一个字符串字面量，且输入本身就是有效的 UTF-8，反序列化器可以直接返回指向原始输入的切片，无需拷贝。

- 这是性能最高的路径（零拷贝）。

```rust
// 伪代码：JSON deserializer 从 &str 解析
if let Some(raw) = self.input.as_str().get(range) {
    return visitor.visit_borrowed_str(raw);
}
```

---

# 2. `visit_str(&str)`

调用时机：反序列化器可以提供一个临时 `&str`，但它的生命周期不满足 `'de` 的约束，或者数据需要经过转义/解码才能得到字符串。

- 例如，从 `String` 的缓冲区中借用，或者从读取的 `Vec<u8>` 中借用一个切片，但这个切片不能和原始输入生命周期关联。

- `Visitor` 需要在这个调用中 立刻消费或复制 这个 `&str` 的内容（例如转为 `String`），不能把引用带出 `visitor`。

- 有些实现也会在已经拥有 `String` 时为了通用性而调用 `visit_str`（把 `String` 当 `&str` 传），但这会失去直接将所有权转移给 `visitor` 的机会。

```rust
let decoded = self.decode_escape()?; // 产生临时 String
visitor.visit_str(&decoded)
```

---

# 3. `visit_string(String)`

调用时机：反序列化器已经拿到一个自有的 `String`，且希望直接把所有权转移给 `visitor`，避免再拷贝一次。

- 比如某些二进制格式中，字符串是 `prefixed length` + `data`，反序列化时必须读出到 `Vec<u8>` 再验证 UTF-8 并生成 `String`。此时直接交给 `visitor` 拥有是最合适的。

- 如果 `visitor` 实现的是 `visit_string`，可以直接获得这个 `String`；如果它没实现，Serde 默认会把 `visit_string` 回退到 `visit_str`（所以调用 `visit_string` 总是安全的）。

---

# 4. `visit_bytes(&[u8])`

在 `deserialize_str` 中标准协议并不调用它。

`deserialize_str` 期望的是 字符串。如果反序列化器把字节直接交给 `visitor`，而 `visitor` 是为 `String` 实现的（它没有实现 `visit_bytes`），就会失败。

但有一种合理例外：

当格式 不区分字符串与字节（如某些二进制格式），且反序列化器 无法确定数据是否为有效 UTF-8 时，它可能会选择在 `deserialize_str` 中调用 `visit_bytes`。这种用法通常出现在需要 宽松处理 的 `deserializer` 实现中（例如允许 `String` 类型也能接收任意字节，配合 serde_bytes）。
这时要求 `visitor` 本身必须实现了 `visit_bytes` 并能将其转换为字符串（例如先检查 UTF-8），否则会报错。

简言之：这是非典型用法，只会在特定容忍字节/字符串不分的格式中出现。

# 5. `visit_borrowed_bytes(&'de [u8])`

同理，在 `deserialize_str` 中标准协议不调用它。
如果真的被调用，那意味着输入是借用给 `visitor` 的字节数组，且反序列化器把 字节流 当作“字符串”呈现。这同样要求 `visitor` 接受字节并能自己判断 UTF-8 有效性（许多 `visitor` 根本不会实现它）。

---

# 6. `visit_byte_buf(Vec<u8>)`

同上，属于 `deserialize_bytes` / `deserialize_byte_buf` 的领域。
除非格式特立独行，把字符串当作自有字节缓冲区传递。

---

# 7. `expecting(&self, f: &mut fmt::Formatter)`

它不是被 `deserialize_str` 直接“调用”的，而是用来生成错误信息。

- 当 `deserialize_str` 发现输入数据不是字符串（比如遇到一个数字或数组），它会返回一个 `Error`。

- `Serde` 的错误生成过程中，会用 `visitor.expecting(f)` 来获取一个描述，说明“反序列化器期望什么类型，却收到了什么”。

- 所以 `expecting` 是由反序列化器在 错误路径 上调用的，用于构造友好的错误提示，例如：

  ```text
  invalid type: integer `42`, expected a string
  ```

这里的 `expected a string` 就是 `visitor.expecting` 的输出。

---

# 总结：协议规定与常见实践

`visitor` 方法	在 `deserialize_str` 中的角色
`visit_borrowed_str`	✅ 首选：可零拷贝借用时
`visit_str`	✅ 可提供临时引用时
`visit_string`	✅ 自有所有权字符串时
`visit_bytes` / `visit_borrowed_bytes` / `visit_byte_buf`	❌ 通常不出现；仅极少数不区分字节/字符串的格式可能使用，且依赖 `visitor` 兼容
`expecting`	🛈 错误处理时用于生成 `expected a string` 等描述，不参与正常值传递

如果你在实现一个 `Deserializer`，记得在 `deserialize_str` 中优先尝试 借用 → 临时引用 → 自有字符串 的顺序，并把字节方法留给 `deserialize_bytes`，这样就能与绝大多数 Serde 生态完美兼容。
