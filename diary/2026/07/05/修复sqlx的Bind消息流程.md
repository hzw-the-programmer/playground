# 1. 修复代码错误

在 bind.rs 中进行两处改动：

## 1.1 修正 body_size_hint 中的长度计算

```rust
fn body_size_hint(&self) -> Saturating<usize> {
    let mut size = Saturating(0);
    size += self.portal.name_len();
    size += self.statement.name_len();

    // 格式码数量（2 字节） + 每个格式码 2 字节
    size += 2;
    size += self.formats.len() * 2;   // 之前是 self.formats.len()

    size += 2; // num_params
    size += self.params.len();

    size += 2;
    size += self.result_formats.len() * 2; // 之前是 self.result_formats.len()

    size
}
```

## 1.2 修正 encode_body 中结果格式码数量的来源

```rust
let result_formats_len = u16::try_from(self.result_formats.len()) // 原来是 self.formats.len()
    .map_err(|_| err_protocol!("too many result format codes ({})", self.result_formats.len()))?;
```

> 同时建议把上一行的 self.formats.len() 的错误消息也顺便修正为参数格式码的描述（目前是 "too many parameter format codes ({})"，已经正确，无需改动）。

---

# 2. 编写单元测试

原代码注释 `// TODO: Unit Test Bind` 给了你一个好理由添加测试。在文件末尾或紧接结构体下方加入：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::PgBufMutExt; // 可能需要根据路径调整
    use std::borrow::Cow;

    #[test]
    fn bind_body_size_matches_encoded() {
        // 构造一个简单的 Bind 消息
        let portal = PortalId::from("p");
        let statement = StatementId::from("s");
        let formats = &[PgValueFormat::Binary, PgValueFormat::Text];
        let params_data = b"\0\0\0\x05hello\0\0\0\x06world!";
        let result_formats = &[PgValueFormat::Binary];

        let bind = Bind {
            portal,
            statement,
            formats,
            num_params: 2,
            params: params_data,
            result_formats,
        };

        // 1) body_size_hint 应与实际编码后的长度一致
        let hint = bind.body_size_hint().0;
        let mut buf = Vec::new();
        bind.encode_body(&mut buf).unwrap();
        assert_eq!(buf.len(), hint, "body_size_hint 必须与实际编码长度一致");

        // 2) 检查关键字节是否符合协议（小范围快照测试）
        // 手动构建期望的十六进制片段，例如检查参数格式码部分
        // 这里只做一个简单的长度检验即可，完整协议合规性可后续补充
    }

    #[test]
    fn bind_format_length_fields() {
        let portal = PortalId::UNNAMED;
        let statement = StatementId::UNNAMED;
        let bind = Bind {
            portal,
            statement,
            formats: &[],
            num_params: 0,
            params: &[],
            result_formats: &[PgValueFormat::Text; 3],
        };

        let mut buf = Vec::new();
        bind.encode_body(&mut buf).unwrap();

        // 确认结果格式码数量字段被正确写入为 3
        // 跳过 portal 和 statement 名称（以 \0 结尾）
        // 然后跳过参数格式码字段，直接定位到结果格式码数量部分
        let mut offset = 0;
        // portal: UNNAMED -> 空字符串 + \0 => 1 字节
        offset += 1;
        // statement: UNNAMED -> 1 字节
        offset += 1;
        // formats_len: u16 = 0 => 2 字节
        offset += 2;
        // num_params: u16 = 0 => 2 字节
        offset += 2;
        // params: 空
        offset += 0;
        // result_formats_len 应在 offset 位置
        assert_eq!(&buf[offset..offset+2], &3u16.to_be_bytes());
    }
}
```

运行 `cargo test -p sqlx-postgres` 确保测试通过。

---

# 3. 贡献流程

## 3.1 Fork 仓库

访问 launchbadge/sqlx 点击右上角 Fork 到你的账户。

## 3.2 创建特性分支

```bash
git clone https://github.com/你的用户名/sqlx.git
cd sqlx
git checkout -b fix-bind-message-encoding
```

## 3.3 应用修改并提交

```bash
# 修改 sqlx-postgres/src/message/bind.rs
git add sqlx-postgres/src/message/bind.rs
git commit -m "fix(postgres): correct Bind message size and format count

- body_size_hint was underestimating the size of format code arrays
  by not multiplying the count by 2 (each i16 takes 2 bytes).
- encode_body incorrectly used self.formats.len() for the number
  of result-column format codes, causing protocol violations when
  the two arrays differ in length.

Added unit tests to verify body_size_hint matches encode_body output
and that the result-formats length field is correctly populated.

Ref: #3464"
```

## 3.4 确保代码风格和质量

```bash
# 运行格式化
cargo fmt --all
# 运行 Clippy
cargo clippy --workspace --all-targets -- -D warnings
# 运行相关测试
cargo test -p sqlx-postgres
```

## 3.5 推送并创建 Pull Request

```bash
git push origin fix-bind-message-encoding
```

在 GitHub 上你的仓库页面会提示创建 PR，点击后填写清晰描述：

标题示例：

```text
fix(postgres): correct Bind message body size hint and result-format count
```

描述模板：

```text
## 问题
当前 `Bind` 消息的 `body_size_hint()` 低估了参数格式码与结果格式码数组所需的字节数（每个元素 2 字节）。同时 `encode_body()` 中写入结果格式码数量时错误地引用了参数格式码数组长度，导致协议包错乱。

相关问题：#3464

## 修改内容
- 将 `body_size_hint` 中 `self.formats.len()` 修正为 `self.formats.len() * 2`，同理处理 `result_formats`。
- 将 `encode_body` 中 `result_formats_len` 的计算改用 `self.result_formats.len()`。
- 添加单元测试，验证 body 大小估算与实际编码一致，以及结果格式数量字段的正确性。

## 验证
- 新增的单元测试通过。
- `cargo clippy` 无警告。
- 所有 PostgreSQL 相关测试套件通过。

## 备注
该修复向后兼容，不会改变任何公开 API，仅修正了内部错误。
```

## 3.6 后续

等待维护者审核。可能他们会要求将测试补充得更完备（例如包含 NULL 参数、多种格式组合的集成测试）。你可以根据反馈迭代更新同一分支。

---

# 4. 注意事项

- SQLx 使用 **Conventional Commits** 提交格式（例如 `fix:` 前缀），请保持统一。

- 如果你修改了 `body_size_hint`，请确认其他消息（如 `Parse`、`Describe`）是否也有类似问题，但本次只聚焦 `Bind`。

- 注释中提及的 `PgValueFormat` 类型实际是 `i16` 的 repr，转换为 `u16` 写入大端序是正确的，无需修改。

- 基准测试（`Benchmark Bind`）是可选的，不在本次修复必须范围，但如果你想挑战，可以用 `criterion` 添加一个简单基准。

完成以上步骤，你的 PR 将是清晰、专业且易于合并的。祝贡献顺利！
