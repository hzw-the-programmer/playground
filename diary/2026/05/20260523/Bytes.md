下面通过几个关键代码示例，具体展示 `Bytes` 和 `BytesMut` 的核心设计：引用计数共享、零拷贝切片、`split_off` 的写时复制，以及如何在实际解析中避免数据拷贝。

---

1. 零拷贝切片与共享：多个 `Bytes` 指向同一块内存

这是 Bytes 最核心的能力：`clone()` 只增加引用计数。`slice()` 先 `clone()`，再调整指针和长度，同样零拷贝。

```rust
use bytes::Bytes;

fn main() {
    // 从一个静态字节串创建 Bytes（注意：这里是借用，实际会产生拷贝，见注释）
    // 为了演示底层内存共享，我们使用 Vec 来确保数据在堆上。
    let data = Bytes::from(vec![b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', b'!']);
    
    // 零拷贝切片：不复制任何字节
    let hello = data.slice(0..5);   // "Hello"
    let world = data.slice(6..11);  // "World"

    // 克隆切片：同样不复制数据，只增加引用计数
    let hello_clone = hello.clone();

    // 三个 Bytes 变量都指向同一块堆内存，只是偏移和长度不同
    // data     -> 偏移 0, 长度 12
    // hello    -> 偏移 0, 长度 5
    // world    -> 偏移 6, 长度 5
    // hello_clone -> 偏移 0, 长度 5，与 hello 共享同一视图

    // 验证：hello 和 hello_clone 的起始指针相同
    assert_eq!(hello.as_ptr(), hello_clone.as_ptr());

    // 即使丢弃原始的 data，hello 和 world 仍然有效
    drop(data);
    println!("hello: {:?}", &hello[..]); // b"Hello"
    println!("world: {:?}", &world[..]); // b"World"
}
```

设计细节：

- `Bytes` 内部包含一个原子引用计数的共享内存块指针、一个偏移和一个长度。

- `clone()` 只增加引用计数，克隆出的实例可以拥有不同的偏移和长度，但共享同一底层内存。

- `slice()` 先 `clone()`，再更改偏移和长度, 所以是 O(1) 操作。

---

2. `BytesMut` 的 `split_off`：可变缓冲区的零拷贝分割

`BytesMut` 是唯一可变的缓冲区，但它的 `split_off` 方法能零拷贝地将缓冲区分成两个独立的 `BytesMut`，内部通过共享内存和写时复制（copy-on-write）来保证安全。

```rust
use bytes::{BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(64);
    buf.put_slice(b"Hello, World!");

    let ptr_before = buf.as_ptr();

    // 在索引 7 处分割（逗号后的空格），buf 保留 "Hello, "，rest 得到 "World!"
    let mut rest = buf.split_off(7);

    // 分割后，两者可能仍然指向同一块底层内存（零拷贝）
    // 注意：as_ptr() 返回的是各自视图的起始地址，由于偏移不同，地址可能不同，
    // 但它们共享同一块分配的内存。
    println!("buf  : {:?}, ptr: {:p}", &buf[..], buf.as_ptr());
    println!("rest : {:?}, ptr: {:p}", &rest[..], rest.as_ptr());

    // 此时对 rest 进行写入，会触发**写时复制**
    // 因为底层内存可能被 buf 共享，所以 rest 会为自己分配一份新的内存，再修改。
    rest.put_u8(b'?');
    let ptr_after_mutation = rest.as_ptr();

    println!("rest after mutation: {:?}, ptr: {:p}", &rest[..], ptr_after_mutation);
    // ptr_after_mutation 很可能与之前不同，表明发生了复制
    assert_ne!(rest.as_ptr(), ptr_before); // 地址已变化（写时复制）
}
```

设计细节：

- `BytesMut` 内部可以是独占的 `Vec<u8>` 或共享的 `Shared` 状态。

- `split_off` 会将内部的 `Kind` 提升为 `Shared`（增加引用计数），并简单地设置两个视图的偏移和长度，因此是 O(1) 零拷贝。

- 当后续对任何共享的 `BytesMut` 进行可变操作（如 `put_u8`）时，如果引用计数 > 1，就会分配新内存并复制，保证不会影响其他视图。

---

3. `freeze` 与 `clone`：从可变到只读的无缝共享

`BytesMut` 通过 `freeze()` 转换为不可变的 `Bytes`，之后就可以安全地在多个地方零拷贝共享。

```rust
use bytes::{BufMut, Bytes, BytesMut};

fn main() {
    let mut builder = BytesMut::with_capacity(32);
    builder.put_slice(b"shared immutable data");

    // freeze 将 BytesMut 转为 Bytes，此后不可变，可以高效共享
    let shared: Bytes = builder.freeze();

    // clone 多次，只增加引用计数，不复制数据
    let handle1 = shared.clone();
    let handle2 = shared.clone();

    // 三者指向同一内存，且指针起始地址相同（因为偏移都是0）
    assert_eq!(shared.as_ptr(), handle1.as_ptr());
    assert_eq!(shared.as_ptr(), handle2.as_ptr());

    // 可以丢弃原始 shared，handle1 和 handle2 依然完好
    drop(shared);
    println!("handle1: {:?}", &handle1[..]);
    println!("handle2: {:?}", &handle2[..]);
}
```

设计细节：

- `freeze()` 会“冻结”底层内存的写入能力，本质是将引用计数与一个只读的 `vtable` 关联，之后所有 `clone()` 都只增加只读引用计数。

这种模式在构建数据帧并分发给多个线程处理时尤为常见，完全避免了数据拷贝。

---

4. 实战：使用 split_to 零拷贝解析数据流

模拟从网络套接字读取数据，并利用 `split_to` 和 `freeze` 提取出完整的帧，剩余数据留在缓冲区中继续使用。

```rust
use bytes::{Buf, Bytes, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(64);
    // 模拟两次数据到达，第二次才构成一个完整帧
    buf.extend_from_slice(b"\x00\x00\x00\x05"); // 4 字节长度前缀，值为 5
    buf.extend_from_slice(b"Hello");             // 5 字节载荷

    // 注意：这里为了演示，假设帧长度已知且数据已全部到达。
    // 实际使用中应检查 buf.remaining() >= 4 + length。

    // 读取长度前缀（4字节大端）
    let len_prefix = buf.split_to(4);
    let length = u32::from_be_bytes(len_prefix[..].try_into().unwrap()) as usize;
    println!("Payload length: {}", length);

    // 按照长度提取出整个帧（零拷贝），剩余数据留在 buf 中
    let frame = buf.split_to(length).freeze();

    // 此时 frame 是只读的 Bytes，buf 可能还残留后续数据
    println!("Frame payload: {:?}", &frame[..]); // b"Hello"
}
```

设计细节：

- `split_to()` 将 `BytesMut` 在指定索引处切开，前半部分作为独立的 `BytesMut` 返回，同时原 `buf` 被缩短。

- 再通过 `freeze()` 将其转为不可变的 `Bytes`，这个过程完全不涉及数据复制，因为 `split_to` 内部通过共享引用计数调整视图实现。

- 这种模式在异步网络编程（如 tokio）中非常常见：从 `TcpStream` 读入数据到 `BytesMut`，解析出帧后直接以 `Bytes` 形式分发，零开销处理。

---

⚠️ 安全注意：立即升级到 bytes 1.11.1 以上

请务必在 `Cargo.toml` 中保持 `bytes` 为最新版本（≥ 1.11.1），并运行 `cargo update`。旧版本存在整数溢出漏洞（CVE-2026-25541 / RUSTSEC-2026-0007），在 release 模式下可能导致未定义行为。

```toml
[dependencies]
bytes = "1.11.1"   # 或更高版本
```

以上所有示例都体现了 bytes 库的设计精髓：通过引用计数和偏移/长度视图，让可变与不可变缓冲区在零拷贝的前提下安全地共享数据，是 Rust 生态中高性能网络应用的基石。
