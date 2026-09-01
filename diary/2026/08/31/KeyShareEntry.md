# TLS KeyShareEntry

`KeyShareEntry` 是 **TLS 1.3** 握手消息里的核心结构体，定义在 RFC 8446 §4.2.8，用于**密钥交换：客户端把自己的 ECDHE / X25519 公钥直接放在 Client Hello，服务端从中选出一个密钥组完成密钥协商**，这是 TLS1.3 实现 1‑RTT 的关键。

## 结构体定义

```plaintext
struct {
    NamedGroup group;
    opaque key_exchange<1..2^16-1>;
} KeyShareEntry;
```

字段说明：

1. **group (2 字节)**：`NamedGroup`，密钥协商算法标识。
   - `0x0017` secp256r1
   - `0x0018` secp384r1
   - `0x001D` X25519（最常用）
   - `0x001E` X448
2. **key_exchange**：可变长度，16 位长度前缀 + 公钥原始字节。
   - X25519：固定 32 字节公钥
   - secp256r1：压缩点 33 字节

> 
> Client Hello 里面是 `KeyShareEntry client_shares<0..2^16‑1>`，即一个 KeyShareEntry 数组。客户端可以携带**多个密钥组的公钥**。

---

## 握手流程中的行为

### Client Hello

客户端发送：

- `supported_groups`：告诉服务端我支持哪些密钥组
- `client_shares`：**KeyShareEntry[]**，客户端预先生成若干密钥组的临时私钥，把对应公钥放进来。

> 
> TLS1.3 核心优化：不再像 TLS1.2 等待 Server Hello 才拿到服务端公钥；客户端直接预先给出公钥候选。

### Server Hello

服务端选择一个客户端提供的 `KeyShareEntry.group`，回复：

```plaintext
struct {
    KeyShareEntry server_share;
}
```

- `server_share.group`：选中的密钥组，必须是客户端 `client_shares` 里出现过的 group
- `server_share.key_exchange`：服务端该组的临时公钥

双方各自用自己临时私钥 + 对方公钥做 DH 运算，得到共享密钥，进入 HKDF 派生主密钥。

> 
> 如果服务端**不支持客户端给出的任何 group**：返回 `HelloRetryRequest`，不返回 server_share，通知客户端重新发送 Client Hello，只携带服务端支持的那一个密钥组。

### HelloRetryRequest（HRR）场景

1. ClientHello 发送一组 KeyShareEntry
2. 服务端没有匹配的 group，返回 HRR，带上 `selected_group`
3. 客户端**重新生成该 group 的临时密钥对**，发送新 ClientHello，只放这一个 KeyShareEntry。

> 
> ⚠️ 不能复用旧私钥，必须重新生成。HRR 会损失 1‑RTT，变成 2‑RTT。

---

## 报文抓包示例（简化）

Client Hello → extensions → key_share

```
Extension: key_share (51)
    Length: 36
    Client Key Shares length: 34
        Key Share Entry
            Group: X25519 (0x001d)
            Key Exchange length: 32
            Key Exchange: [32 bytes X25519 client public key]
```

Server Hello → extensions → key_share

```
Extension: key_share (51)
    Length: 34
    Server Key Share
        Group: X25519 (0x001d)
        Key Exchange length:32
        Key Exchange: [32 bytes X25519 server public key]
```

---

## 关键细节与坑点

1. **key_exchange 是原始公钥，没有 ASN.1 包装**
X25519 直接输出 32 字节，不要做 DER 编码。很多 TLS 实现错误在这里。
2. NamedGroup≠CipherSuite
TLS1.3 CipherSuite 只负责对称加密 + HMAC；密钥交换完全由 `KeyShareEntry / NamedGroup` 决定。
3. 客户端可以携带多个 KeyShareEntry，但每个 group 最多出现一次。
比如同时带上 X25519 + secp256r1，服务端选其一。
4. 没有 RSA 密钥交换！
TLS1.3 彻底移除静态 RSA 密钥交换，所有握手都基于 DHE/ECDHE，全部提供前向保密 (PFS)。
5. 内存与安全

- KeyShareEntry 里面的 `key_exchange` 是**公钥**，不需要保密；临时私钥必须用完立刻销毁。
- HRR 流程必须丢弃旧临时私钥，生成全新密钥对，防止侧信道。

---

## Rust/Tokio‑rustls 视角

`rustls` 中对应类型：

- `rustls::msgs::handshake::KeyShareEntry`
- `rustls::NamedGroup`

客户端构建 ClientHello：预生成 `KeyShareEntry` 数组放入 `KeyShare` extension。
服务端遍历 client_shares，和自己的 `supported_groups` 求交集，选最优 group，输出 server_share；无交集返回 HRR。

伪代码示意：

```rust
pub struct KeyShareEntry {
    pub group: NamedGroup,
    pub key_exchange: Vec<u8>,
}
```

---

## 和 TLS1.2 的对比

表格

| 项目         | TLS1.2                        | TLS1.3 KeyShareEntry                |
| ------------ | ----------------------------- | ----------------------------------- |
| 公钥交换时机 | Server Hello 才下发服务端公钥 | Client Hello 直接携带客户端公钥候选 |
| RTT          | 2‑RTT                         | 1‑RTT（正常） / 2‑RTT（HRR 回退）   |
| PFS          | ECDHE 可选                    | 强制全部 PFS                        |
| 载体         | ServerKeyExchange 消息        | Hello 的 extension `key_share(51)`  |
