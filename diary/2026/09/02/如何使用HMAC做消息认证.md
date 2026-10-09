# 如何使用 HMAC 做消息认证

HMAC 提供两件事：**完整性（消息没被篡改）** + **数据源认证（对方持有相同密钥）**。

> 
> ⚠️ HMAC **不加密消息本身**，消息仍是明文；只是附带一段校验标签 (tag)。如果需要保密，要搭配加密（例如 AEAD）。

完整流程分两方：**发送方**、**接收方**，前提：双方提前安全共享同一个 HMAC 密钥 `K`。

## 整体流程

### 发送方：生成 HMAC tag

输入：

1. 共享密钥 `K`（字节，密钥必须安全分发，不能走传输信道）
2. 原始消息 `M`（可以任意字节：二进制、字符串）

步骤：

1. 选定底层哈希算法：SHA‑256 / SHA‑384（TLS1.3 使用 SHA‑256）
2. 计算 `tag = HMAC‑Hash(K, M)`
3. 把 **原始消息 M + tag** 一起发给接收方

> 
> 传输格式示例：`M || tag` 或者 JSON `{"msg":"xxx","hmac_tag":"hex字符串"}`

> 
> 重点：**消息 M 明文传输，tag 是认证凭证**。攻击者可以篡改 M，但无法算出合法 tag，因为没有密钥 K。

### 接收方：校验 HMAC

收到：消息 M'，收到的 tag_rcv

1. 使用**完全相同的密钥 K、相同哈希算法**，本地计算 `tag_local = HMAC‑Hash(K, M')`
2. **恒定时间对比 constant‑time compare**：比较 `tag_local` 和 `tag_rcv`
   - 相等：认证通过，消息可信
   - 不相等：消息被篡改 / 密钥不对 / 攻击者伪造，直接丢弃，禁止继续业务处理

> 
> ❗严禁普通字符串相等（`==`）做对比，会引入时序攻击。密码库的 verify API 内部自带恒定时间校验，优先使用库提供的 verify，不要自己手写字节循环比较。

---

## 安全前置条件（缺一不可）

1. **密钥 K 必须安全共享**：HMAC 安全全部依赖密钥 K；密钥泄露 = 整个认证彻底失效。
HMAC 不解决密钥分发问题，TLS 中密钥 K 来自 HKDF 派生出来的 traffic_secret。
2. 密钥长度建议：不短于哈希输出长度，SHA‑256 场景建议密钥≥32 字节。
3. 哈希算法不要用 MD5、SHA‑1，只使用 SHA‑256 / SHA‑384。
4. **必须抵抗重放攻击**：HMAC 本身**不能防御重放**。

> 
> 攻击者把合法的 (M,tag) 原封不动重复发送，HMAC 校验依然会通过。
> 解决：消息中嵌入 **nonce / 序列号 / 时间戳**，把它作为 M 的一部分参与 HMAC 计算。

> 
> 示例消息构造防重放：`M = seq || actual_payload`，序列号 seq 每次递增，参与 HMAC。

## 实操示例

### Rust ring 库（标准生产写法）

```rust
use ring::hmac;
use hex;

fn main() {
    // 1. 双方预共享密钥K
    let key_data = b"32bytes_secretkey_________123456";
    let key = hmac::Key::new(hmac::HMAC_SHA256, key_data);

    // ========== 发送方：生成tag ==========
    let payload = b"hello tls hmac message";
    let tag = hmac::compute(&key, hmac::Input::from(payload));
    println!("hmac tag hex = {}", hex::encode(tag.as_ref()));

    // 发送：payload + tag.as_ref()

    // ========== 接收方校验（正确情况） ==========
    match hmac::verify(&key, payload, tag.as_ref()) {
        Ok(_) => println!("✅ HMAC校验通过"),
        Err(_) => println!("❌ 消息篡改或者密钥错误"),
    }

    // ========== 模拟消息被篡改 ==========
    let tampered_msg = b"hello tls hmac hacked";
    match hmac::verify(&key, tampered_msg, tag.as_ref()) {
        Ok(_) => println!("✅ HMAC校验通过"),
        Err(_) => println!("❌ 检测到消息篡改"),
    }
}
```

> 
> `hmac::verify` 内部已经是恒定时间比较，不要取出字节手动`==`。

### Python 示例

```python
import hmac
import hashlib

key = b"32bytes_secretkey_________123456"
msg = b"hello tls hmac message"

# 发送方生成tag
tag = hmac.new(key, msg, hashlib.sha256).digest()

# 接收方校验，使用compare_digest恒定时间对比
rcv_msg = b"hello tls hmac message"
rcv_tag = tag
if hmac.compare_digest(hmac.new(key, rcv_msg, hashlib.sha256).digest(), rcv_tag):
    print("✅ 校验通过")
else:
    print("❌ 校验失败")
```

---

## TLS1.3 中 HMAC 消息认证实例：Finished 报文

```
verify_data = HMAC‑SHA256(traffic_secret, transcript_hash)
```

- `K = traffic_secret`：HKDF‑Expand 派生得到的握手密钥
- `M = transcript_hash`：从握手开始到此为止全部握手消息的累积哈希
- 发送 Finished：把 verify_data 放在 Finished 报文内；整个 Finished 报文本身还会被 AEAD 加密保护。
- 对端：使用同一份 traffic_secret，重新计算 transcript_hash，重新计算 verify_data，做 HMAC verify。

> 
> Finished 的语义：证明对方完整走完密钥调度，持有相同 secret，握手报文没有被中间人篡改。

---

## 常见错误做法

1. ❌ 只对部分消息做 HMAC，漏掉序列号、元数据；攻击者可以替换元数据。

> 
> ✅ 所有需要完整性保护的字段全部放进 M 参与 HMAC 计算。

2. ❌ 传输只发送 tag，不发送原始消息 M。HMAC 不会恢复消息，接收方必须拿到完整 M。
3. ❌ 使用普通相等比较 tag，造成时序攻击。

> 
> ✅ 一定调用库的 verify /compare_digest。

4. ❌ 依赖 HMAC 阻止重放攻击。HMAC 没有这个能力，必须自己加序列号 /nonce。
5. ❌ 密钥太短，例如密钥只有几字节，暴力破解风险。
6. ❌ 混淆参数顺序：`HMAC(key, message)`，不要写成 `HMAC(message, key)`。

---

## HMAC 与 AEAD 的选用区分

1. **只需要认证、不需要加密** → HMAC (M)，传输 M+tag。
2. **既需要加密，又需要认证** → 优先 AEAD（AES‑GCM / ChaCha20‑Poly1305）。

> 
> AEAD 同时产出密文 + 认证 tag；明文不会出现在传输信道。TLS 记录层使用 AEAD，握手 Finished 使用 HMAC。
