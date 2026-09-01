# HKDF 详解：HKDF‑Extract + HKDF‑Expand

HKDF = HMAC‑based Key Derivation Function，基于 HMAC 的密钥派生函数，RFC‑5869；TLS1.3 强制使用 HKDF，SHA256 / SHA384。

> 
> 整体两阶段：
> 
> 
> 1. **Extract 提取：把零散输入密钥材料压缩成统一长度伪随机密钥 PRK**
> 2. **Expand 扩展：把短 PRK，扩展输出任意长度的密钥字节串 derived_output**

```plaintext
IKM (输入密钥材料) + salt ──▶ HKDF‑Extract ──▶ PRK
PRK + info(上下文标签) ──▶ HKDF‑Expand ──▶ OKM(输出密钥材料)
```

缩写约定：

- IKM：Input Keying Material，输入密钥材料，原始、可能熵分布不均匀的字节（TLS 中就是 ECDH 输出 Z）
- salt：盐，HMAC 的 key；可以是 0 长度字节串
- PRK：Pseudo‑Random Key，Extract 输出，固定等于 Hash 输出长度（SHA‑256 → 32 字节；SHA‑384 →48 字节）
- info：上下文标识，用于区分不同用途密钥，**防止不同场景密钥复用混淆**，TLS1.3 大量使用如`b"tls13 derived"`、`b"tls13 handshak"`
- OKM：Output Keying Material，Expand 输出，就是我们代码里叫的`derived`。

---

## 1. HKDF‑Extract(salt, IKM) → PRK

底层就是一次 HMAC。

```plaintext
PRK = HMAC‑Hash(salt, IKM)
```

### 重要细节

1. 如果 `salt` 是空字节串 `[]`，RFC5869 规定：替换为一串等于 Hash 输出长度的 0 字节。

> 
> SHA256 场景，salt 为空等价于 salt = [0u8;32]

2. IKM 可以任意长度，比如 ECDH 输出 Z 是 32 字节。Extract 不做长度校验，直接喂给 HMAC。
3. **Extract 输出 PRK 长度固定，等于哈希摘要长度**
   - SHA2‑256：PRK = 32 字节
   - SHA2‑384：PRK = 48 字节

> 
> Extract 的核心目标：**把来源杂乱、长度不定、熵分布不均匀的 IKM，整理成一个密码学质量均匀的固定长度 PRK。**

> 
> TLS1.3 例子：
> `handshake_secret = HKDF‑Extract(early_derived, Z)`
> 
> 
> - salt = early_derived
> - IKM = ECDH 共享输出 Z

⚠️ 不要把 PRK 直接拿来当业务密钥，PRK 只给 Expand 做输入。

---

## 2. HKDF‑Expand(PRK, info, L) → OKM（derived）

> 
> L：期望输出字节长度；上限：`255 * HashLen`，SHA256 最大 255*32=8160 字节。

算法迭代逻辑，伪代码：

```plaintext
HashLen = HMAC输出字节数
N = ceil(L / HashLen)

T_0 = b""
T_1 = HMAC‑Hash(PRK, T_0 || info || 0x01)
T_2 = HMAC‑Hash(PRK, T_1 || info || 0x02)
T_3 = HMAC‑Hash(PRK, T_2 || info || 0x03)
...
T_N = HMAC‑Hash(PRK, T_{N‑1} || info || N )

OKM = concat(T_1 || T_2 || T_3 ... ) 截取前L字节
```

关键点拆解：

1. HMAC 的**密钥固定是 PRK**，全程 Expand 阶段 PRK 不变。
2. 每一轮输入 = 上一轮输出 || info || 计数器单字节 (1,2,3...)
3. info 会被**每一轮 HMAC 都带上**，实现上下文隔离。
4. 计数器防止多块输出重复；最大 255，所以 L 有上限。
5. 输出 OKM 就是 TLS 里代码变量常叫的`derived`。

> 
> 示例 TLS1.3

```plaintext
handshake_secret(PRK,32B)
HKDF‑Expand(handshake_secret, b"tls13 handshak", 32)
→ client_handshake_traffic_secret (OKM，32字节derived)
```

---

# 📌 TLS1.3 HKDF 完整链路对照（SHA256）

RFC8446 第 7 章密钥调度

```plaintext
early_secret        = HKDF‑Extract(zero‑salt, zero‑ikm)
early_derived       = HKDF‑Expand(early_secret, b"tls13 derived", 32)

handshake_secret    = HKDF‑Extract(salt=early_derived, IKM=Z)
client_handshake_traffic_secret = HKDF‑Expand(handshake_secret, b"tls13 c hs traffic",32)
server_handshake_traffic_secret = HKDF‑Expand(handshake_secret, b"tls13 s hs traffic",32)

master_secret       = HKDF‑Expand(handshake_secret, b"tls13 derived", 32)
client_application_traffic_secret_0 = HKDF‑Expand(master_secret, b"tls13 c ap traffic",32)
server_application_traffic_secret_0 = HKDF‑Expand(master_secret, b"tls13 s ap traffic",32)
```

> 
> 注意：`master_secret` 不是 Extract 出来的，**是 Expand 输出的 OKM**，很多人会搞错。

---

## Extract vs Expand 对比表

表格

|          | HKDF‑Extract                        | HKDF‑Expand                            |
| -------- | ----------------------------------- | -------------------------------------- |
| 输入     | salt + IKM                          | PRK + info + L (输出长度)              |
| 输出     | PRK，固定长度 = HashLen             | OKM (derived)，可自定义长度 L          |
| 底层原语 | HMAC‑Hash(salt, IKM)                | 多轮迭代 HMAC‑Hash (PRK, ...)          |
| 作用     | 把原始密钥材料 “提纯压缩” 成 PRK    | 从 PRK 派生出多个不同用途密钥          |
| TLS 角色 | 产生 early_secret、handshake_secret | 产生所有 traffic_secret、master_secret |

---

## 容易踩坑点

1. ❌ 混淆两个 HMAC 参数顺序`Extract( salt, IKM )` = `HMAC(salt /*key*/, IKM /*data*/)`
很多代码写反，直接结果全部错误。
2. ❌ info 传错字符串，TLS1.3 的 info 是特定 ASCII 字节，末尾不带`\0`，大小写严格。`b"tls13 derived"` 不能写成 `"tls13_derived"`。
3. ❌ Expand 输出长度超限，L > 255*HashLen，密码库直接报错。
4. ❌ 把 Extract 输出 PRK 直接用作加密密钥；PRK 只允许作为 Expand 输入。
5. ✅ info 核心价值：同一 PRK，不同 info 得到完全不同 derived 输出，实现密钥隔离。

> 
> 同一个 handshake_secret，info=`tls13 c hs traffic`得到客户端握手密钥；info=`tls13 s hs traffic`得到服务端握手密钥。

---

## Rust 极简可运行示例 (hkdf crate + sha2)

```rust
use hkdf::Hkdf;
use sha2::Sha256;
use hex;

fn main(){
    // 1. HKDF‑Extract
    let salt = b"salt_example";
    let ikm = b"ecdh_z_input_material";
    let prk = Hkdf::<Sha256>::extract(Some(salt), ikm);
    println!("PRK hex: {}",hex::encode(prk.as_prk()));

    // 2. HKDF‑Expand → derived(OKM)
    let hkdf = prk;
    let info = b"tls13 derived";
    let mut derived = [0u8;32];
    hkdf.expand(info, &mut derived).unwrap();
    println!("derived OKM hex: {}",hex::encode(&derived));
}
```

> 
> `Hkdf::extract()` 对应 HKDF‑Extract；`.expand()`对应 HKDF‑Expand。
