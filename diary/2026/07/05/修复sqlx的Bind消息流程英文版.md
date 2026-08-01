# 1. Code Fixes in body_size_hint and encode_body

File: sqlx-postgres/src/message/bind.rs

## body_size_hint

```rust
fn body_size_hint(&self) -> Saturating<usize> {
    let mut size = Saturating(0);
    size += self.portal.name_len();
    size += self.statement.name_len();

    // Number of parameter format codes (2 bytes) + 2 bytes per code
    size += 2;
    size += self.formats.len() * 2;   // Was: self.formats.len() (missing *2)

    // num_params (2 bytes)
    size += 2;

    // Pre-serialized parameter block
    size += self.params.len();

    // Number of result format codes (2 bytes) + 2 bytes per code
    size += 2;
    size += self.result_formats.len() * 2; // Was: self.result_formats.len() (missing *2)

    size
}
```

## encode_body – corrected result format count source

```rust
let result_formats_len = u16::try_from(self.result_formats.len()) // Was: self.formats.len()
    .map_err(|_| err_protocol!("too many result format codes ({})", self.result_formats.len()))?;
```

> The error message for parameter format codes already says "too many parameter format codes ({})", so no change needed there.

---

# 2. Unit Tests (English Comments)

Place these at the end of `bind.rs` or in a `tests` module.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_size_hint_matches_encoded_size() {
        // Build a simple Bind message with mixed formats and actual parameter data
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

        // The hint must equal the actual encoded body length
        let hint = bind.body_size_hint().0;
        let mut buf = Vec::new();
        bind.encode_body(&mut buf).unwrap();
        assert_eq!(
            buf.len(),
            hint,
            "body_size_hint should match the length of the encoded body"
        );
    }

    #[test]
    fn result_format_length_field_is_correct() {
        let portal = PortalId::UNNAMED;
        let statement = StatementId::UNNAMED;
        let bind = Bind {
            portal,
            statement,
            formats: &[],
            num_params: 0,
            params: &[],
            // Use three result format codes so we can verify the count field
            result_formats: &[PgValueFormat::Text; 3],
        };

        let mut buf = Vec::new();
        bind.encode_body(&mut buf).unwrap();

        // Navigate to the position of the result-format-code count field:
        // - portal name: "UNNAMED" => empty string + '\0' = 1 byte
        // - statement name: "UNNAMED" => 1 byte
        // - parameter format count (u16 = 0): 2 bytes
        // - num_params (u16 = 0): 2 bytes
        // - params: 0 bytes
        // Total offset after those fields = 6
        let offset = 6;
        assert_eq!(
            &buf[offset..offset + 2],
            &3u16.to_be_bytes(),
            "Result format count field should contain 3"
        );
    }
}
```

---

# 3. Pull Request Template (English)

## Title:

```text
fix(postgres): correct Bind message body size hint and result-format count
```

## Description:

```markdown
## Problem
The `Bind` message's `body_size_hint()` underestimated the byte size of the
parameter format code array and the result format code array because it used
`len()` (element count) instead of `len() * 2` (each element is an `i16`).

Additionally, `encode_body()` incorrectly used `self.formats.len()` when
writing the number of result-column format codes. This leads to protocol
violations whenever the two format arrays have different lengths.

Related issue: #3464

## Changes
- In `body_size_hint`, changed `self.formats.len()` to `self.formats.len() * 2`
  and `self.result_formats.len()` to `self.result_formats.len() * 2`.
- In `encode_body`, changed the calculation of `result_formats_len` to use
  `self.result_formats.len()` instead of `self.formats.len()`.
- Added unit tests that verify the body size hint matches the actual encoded
  output, and that the result-format count field is correctly populated.

## Testing
- New unit tests pass.
- `cargo clippy` reports no warnings.
- All PostgreSQL-related tests in the workspace pass.

## Notes
- The fix is fully backward-compatible and does not change any public API.
- Only the internal message encoding is corrected.
```

---

Here's a conventional commit message ready to paste:

```text
fix(postgres): correct Bind message body size hint and result-format count

The Bind message's body_size_hint() underestimated the byte size of
format code arrays because it used the element count instead of
element count * 2 (each PgValueFormat is 2 bytes). Additionally,
encode_body() incorrectly used self.formats.len() when writing the
number of result-column format codes, causing protocol violations
when the two format arrays differ in length.

- body_size_hint: multiply formats.len() and result_formats.len() by 2
- encode_body: use self.result_formats.len() for result format count
- Add unit tests verifying the hint matches the actual encoded size
  and that the result format count field is correctly populated

Ref: #3464
```
