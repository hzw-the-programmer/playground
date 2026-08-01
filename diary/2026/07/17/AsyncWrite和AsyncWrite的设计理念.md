# 一、核心定位区分

## 1. `trait AsyncWrite`：底层基础契约（最小接口）

是**最小化、无依赖、可实现**的底层 trait，只定义异步写入的核心原语，不封装任何便利逻辑，留给 IO 实现者（文件、TcpStream、管道、内存 buffer）去实现。

## 2. `trait AsyncWriteExt`：扩展便利 trait（语法糖层）

给所有 `impl AsyncWrite` 的类型自动实现，提供批量、易用的高层封装，**不新增底层 IO 逻辑**，全部基于 AsyncWrite 的基础方法组合而成。

# 二、AsyncWrite 底层 trait 设计理念：最小正交、无隐藏分配

## 1. 核心方法（标准定义）

```rust
// 写入分片，返回写入字节数，允许部分写入
fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>>;
// 批量分片写入（iovec，零拷贝场景）
fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>]) -> Poll<Result<usize>>;
// 冲刷缓冲区，保证数据落盘/发送
fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>;
// 关闭写入端（TCP 半关闭、文件结束）
fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>;
```

设计思想

1. Poll 模型，贴合 Rust 异步无 runtime 绑定

采用 `poll_*` 同步唤醒模型，不绑定 tokio/async-std，是 `futures-io` 标准抽象，任何异步运行时都能兼容。

2. 允许部分写入（和 std::io::Write 保持语义对齐）

`poll_write` 不保证写完整个 buf，返回 `n < buf.len()` 是合法状态；上层必须循环重试，把复杂循环交给 AsyncWriteExt 封装，底层只做最简 IO。

3. 分离 write /flush/shutdown，职责单一

  - write：仅拷贝数据到内核 / 用户缓冲区；
  - flush：强制缓冲区数据下发，等待 IO 完成；
  - shutdown：优雅关闭写通道（TCP FIN、文件句柄收尾）。

  三者分开，给底层实现完全控制权：缓冲策略、延迟发送、半关闭逻辑由设备自己决定，抽象不做强制假设。

4. write_vectored 可选优化，不破坏最小接口

提供分散写入系统调用（writev）的钩子，但有默认空实现；实现者可按需优化，不强制所有 IO 类型实现，兼顾性能与实现成本。

5. Pin 约束：适配自引用缓冲

`Pin<&mut Self>` 约束，允许 AsyncWrite 实现内部持有自引用缓冲区（如带内部 Vec 的 BufWriter），符合 Rust 异步内存安全模型。

# 三、AsyncWriteExt 扩展 trait 设计理念：统一高层 API，降低使用成本

## 1. 关键方法（全部基于 poll_* 循环封装）

- write(&mut self, buf: &[u8]) -> impl Future<Result<usize>>：poll_write
- write_all(&mut self, buf: &[u8]) -> impl Future<Result<()>>：循环重试，完整写完整个 buffer，屏蔽部分写入细节
- write_vectored：批量分片写入
- flush() / shutdown()：异步 Future 封装，不用手动 poll
- write_u8 / write_u32 / write_all_buf 等工具方法

核心设计思路

1. 扩展 trait 孤儿规则方案：一劳永逸赋能所有实现

Rust 孤儿规则限制无法为外部 trait 给外部类型加方法。因此拆分为两层：

- 底层：第三方设备实现 AsyncWrite；
- 上层：AsyncWriteExt blanket impl T: AsyncWrite，所有实现自动获得全套便利方法，不用每个 IO 类型重复实现 write_all、循环逻辑。

2. 屏蔽底层 poll 复杂细节，面向业务开发

业务代码几乎不会直接调用 poll_write，只使用 .write_all().await；扩展层封装循环、唤醒、重试逻辑，分离「实现者」和「使用者」的复杂度：

- 设备作者：只实现底层 poll 原语；
- 业务开发者：直接调用 async 异步方法，不用手动处理 Context、Poll、部分写入重试。

3. 不侵入底层行为，无额外缓冲、无隐藏分配

AsyncWriteExt 只是逻辑封装，不会私自加缓冲区、不会自动 flush；所有 IO 行为完全遵循 AsyncWrite 实现本身的缓冲策略，保证行为可预测。

4. 和同步 std::io::Write API 语义对齐，降低迁移成本

同步标准库有 Write + WriteExt，异步镜像这套分层设计：

- std::io::Write ↔ AsyncWrite（底层原语）
- std::io::WriteExt ↔ AsyncWriteExt（高层便利方法）

同步转异步学习成本极低，接口命名、行为逻辑保持一致。

# 四、分层设计的整体优势（为什么不合并成一个 trait）

1. 关注点分离

- AsyncWrite：面向 IO 设备实现者（TcpStream、File、BufWriter、Pipe），追求最小接口、最小实现负担；
- AsyncWriteExt：面向业务使用者，追求易用、完整、批量操作。

2. 灵活性 vs 便捷性解耦

若把 write_all、flush future 全部塞进 AsyncWrite：

- 每个自定义 IO 都要重复实现大量循环逻辑，冗余；
- trait 方法爆炸，接口臃肿，底层实现负担极大。

拆分后底层仅 4 个 poll 方法，极简。


3. 兼容零开销抽象

AsyncWriteExt 所有方法都是基于 AsyncWrite 的组合器，无堆分配、无运行时开销，只是语法糖，不影响 IO 性能。

4. 可分层包装装饰器（BufWriter 等中间层）

缓冲层 `BufWriter<W: AsyncWrite>` 只需要实现 AsyncWrite，就能自动继承 AsyncWriteExt 全部方法；装饰器模式畅通无阻，不用重新实现便利 API。

# 五、典型使用分层示例

```rust
// 底层设备：TcpStream 只实现 AsyncWrite（poll_write/poll_flush...）
let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;

// 直接使用 AsyncWriteExt 扩展方法，自动可用
stream.write_all(b"hello world").await?;
stream.flush().await?;
stream.shutdown().await?;
```

# 六、总结一句话设计核心

1. AsyncWrite：定义异步写入的最小、底层、可 poll 的基础契约，面向 IO 实现者，追求极简、通用、无运行时绑定；

2. AsyncWriteExt： blanket 扩展 trait，基于底层原语封装高频易用的异步批量写入逻辑，面向业务开发者，屏蔽 poll 与部分写入的复杂细节；

3. 整体采用分层分离思想，复刻同步 std::io 的分层范式，兼顾底层实现灵活性与上层开发便捷性，零额外性能损耗。
