# HMAC 详解（RFC‑2104）

HMAC = Hash‑based Message Authentication Code，**基于哈希的消息认证码**。
它不是加密算法，是**消息完整性 + 消息来源认证**算法：证明消息没有被篡改、且持有同一个密钥的一方生成。
TLS1.3 底层：HKDF 的全部内部原语就是 HMAC‑SHA256 / HMAC‑SHA384；Finished 报文 verify_data 也是 HMAC。

公式符号：

> 
> \(HMAC(K, M) = H\big[(K \oplus opad) \;\Vert\; H\big((K \oplus ipad)\Vert M\big)\big]\)

- H：底层哈希函数（SHA‑256、SHA‑384）
- K：密钥（HMAC key）
- M：待认证消息
- ipad：inner‑padding 内部填充字节常量
- opad：outer‑padding 外部填充字节常量
- \(\oplus\)：按字节异或
- \(\Vert\)：字节串拼接

## 常量定义

```plaintext
ipad = 0x36 重复 block_size 次
opad = 0x5c 重复 block_size 次
```

> 
> block_size 是哈希算法**内部块大小**，不是摘要输出长度，这点极易混淆。

表格

| 哈希    | 输出摘要长度 (digest‑len) | 内部块大小 block‑size |
| ------- | ------------------------- | --------------------- |
| SHA‑256 | 32 字节                   | 64 字节               |
| SHA‑384 | 48 字节                   | 128 字节              |
| SHA‑512 | 64 字节                   | 128 字节              |

> 
> ⚠️ SHA256：输出 32 字节，但内部处理块是 64 字节。

## HMAC 完整执行步骤

1. **密钥预处理 K → K0**

> 
> 得到等长于 block_size 的 K0。
   1. 如果密钥 K 长度 > block_size：先对 K 做一次哈希，把哈希结果作为新密钥；
   2. 如果密钥 K 长度 < block_size：在密钥末尾补 0x00，补齐到 block_size 字节。
2. 内层计算 inner‑hash
   - \(K0 \oplus ipad\)：K0 每一字节和`0x36`异或
   - 拼接消息 M：`(K0 ⊕ ipad) || M`
   - 对拼接结果做哈希：\(H_1 = H\big((K0\oplus ipad)\Vert M\big)\)
3. 外层计算 outer‑hash
   - \(K0 \oplus opad\)：K0 每一字节和`0x5c`异或
   - 拼接内层哈希结果 \(H_1\)：`(K0 ⊕ opad) || H₁`
   - 再哈希一次得到最终输出：

     \(\boldsymbol{HMAC(K,M)} = H\big((K0\oplus opad)\Vert H_1\big)\)

输出长度 = 底层哈希摘要长度，SHA‑256 → 32 字节 HMAC 输出。

---

# 关键概念区分

1. **HMAC ≠ Hash**

- Hash：单向，无密钥；任何人拿到消息都可以算 hash，无法认证身份。
- HMAC：带密钥 K；只有持有 K 的双方，才能算出相同 MAC 值。

> 
> 用途：完整性 + 认证；**不能提供保密性，消息 M 本身是明文**。

2. HMAC 输出叫 MAC /tag。
接收方：拿到 M+tag，使用相同 K 重新计算 HMAC，对比输出是否完全相等。

> 
> ⚠️ 必须**固定时间比较（constant‑time compare）**，不能直接普通`==`，防止时序攻击。TLS 库全部使用 constant‑time 校验。

## 和 HKDF 的关系（TLS1.3 视角）

> 
> HKDF‑Extract 本质就是一次 HMAC：

\(PRK = HMAC_{Hash}(salt,\ IKM)\)

- HKDF‑Extract：`salt` 充当 HMAC 的密钥 K；`IKM`充当消息 M。

> 
> HKDF‑Expand 内部每一轮 Tₙ也全部调用 HMAC：

\(T_i = HMAC_{Hash}(PRK,\ T_{i‑1} \Vert info \Vert i)\)

- Expand 阶段：PRK 固定作为 HMAC 的密钥 K；变化的是输入消息部分。

> 
> TLS1.3 Finished 报文 verify_data：

\(verify\_data = HMAC‑SHA256(traffic\_secret,\ transcript\_hash)\)
这里 `traffic_secret` 是 HMAC 密钥 K，transcript_hash 是消息 M。

❗非常容易搞混参数顺序：

```
HMAC( K /*密钥*/ , M /*消息*/ )
HKDF‑Extract(salt, IKM) = HMAC(salt, IKM)
👉 salt是HMAC的key，IKM是HMAC的data。不要写反！写反输出完全错误。
```

## 举一个极简字节示意（SHA‑256，block_size=64）

假设输入密钥 K 短于 64 字节：

1. K 后面补 0x00，填充到 64 字节得到 K0。
2. inner_input = (K0 XOR 0x36 重复 64 次) || M
3. h1 = SHA256(inner_input)
4. outer_input = (K0 XOR 0x5c 重复 64 次) || h1
5. hmac_tag = SHA256 (outer_input) → 32 字节输出。

## 常见踩坑清单

1. ❌混淆哈希输出长度 vs block_size。SHA256 输出 32 字节，块大小 64 字节；密钥预处理按 block_size，不是摘要长度。
2. ❌参数顺序颠倒：HMAC (key, message)，不要写成 HMAC (message, key)，TLS HKDF 会直接全部错误。
3. ❌普通字符串比较 MAC tag，造成时序侧信道攻击，密码库必须使用 constant‑time 相等校验。
4. ❌把 HMAC 当成加密：HMAC 只做认证完整性，不会加密消息本身。如果需要加密 + 认证，用 AEAD（TLS 中 AES‑GCM）。
5. ❌密钥超长时忘记做 hash 压缩：标准 HMAC 会自动把长密钥先 hash；自己手写实现必须处理该分支。

## Rust 示例（ring /hmac‑sha256）

```rust
use ring::hmac;
use hex;

fn main(){
    let key_bytes = b"my_secret_key_123";
    let key = hmac::Key::new(hmac::HMAC_SHA256, key_bytes);
    let msg = b"transcript_hash_data_here";

    let tag = hmac::compute(&key, hmac::Input::from(msg));
    println!("hmac tag hex: {}", hex::encode(tag.as_ref()));

    // 校验：constant‑time verify
    hmac::verify(&key, msg, tag.as_ref()).unwrap();
}
```

## HMAC、HKDF、TLS Finished 三者链路小结

```plaintext
HMAC( K, M )
    ↓作为原语
HKDF‑Extract(salt,IKM)=HMAC(salt,IKM)
HKDF‑Expand内部循环调用HMAC(PRK, ...)
    ↓输出 traffic_secret
Finished.verify_data = HMAC(traffic_secret, transcript_hash)
```
