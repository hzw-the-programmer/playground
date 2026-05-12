以下计划专为已掌握 Rust 基础、接触过多线程但想系统吃透内存排序的开发者设计。整体约需 5~7 周（每天 1–1.5 小时），边学边练，重在“从模型到代码，再从代码回到硬件”。

---

# 学习准备：你需要哪些底子

- Rust 并发基础：熟悉 Arc、Mutex、thread::spawn、Send + Sync trait。
- 原子操作概念：知道 AtomicBool、AtomicUsize 的 load、store、compare_exchange 等方法签名。
- 工具链：能轻松写多线程测试，用 cargo run/test 和 loom（建议安装）。
- 心态：内存序很难单靠“读完记下”搞定，必须配合大量实验和失败。

---

# 第一阶段：原子类型与五种 Ordering 初览（第 1 周）

## 目标

能正确调用原子方法，理解五种排序字面含义，建立“排序只影响其他线程的观测顺序”这一核心认知。

## 任务清单

1. 官方文档通读
   
   - 打开 [std::sync::atomic::Ordering 文档](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html) 并对照 [nomicon 的 Atomics 章节](https://doc.rust-lang.org/nomicon/atomics.html)。
   - 重点记住：Relaxed、Acquire（对 load）、Release（对 store）、AcqRel（对 compare_exchange 等读-改-写操作）、SeqCst。

2. 单线程实验：验证都不乱序（故意的）

   - 写一个简单的 fetch_add 代码，在主线程里不停用不同 Ordering 操作，观察在单一线程里都一样。
   - 意义：强化“排序只关乎跨线程可见性”的概念。

3. 可视化朴素理解

  - 画一个表：
    
    Relaxed – 原子性保证，无顺序保证
    Acquire – 后面的读写不能重排到此操作之前
    Release – 前面的读写不能重排到此操作之后
    AcqRel – 同时具有 Acquire 和 Release 的屏障效果
    SeqCst – 全局一致的顺序（所有线程看到同一总顺序）

## 产出

能写出一个简单多线程程序，在每个 load/store 上明确写出 Ordering 而不报错，并能说出每个变体对“重排”的约束方向。

---

# 第二阶段：Relaxed – 唯一只有原子性没有同步的排序（第 2 周）

## 目标

彻底吃透 Relaxed 的适用范围：共享计数器、进度指示器，以及为什么它不能用来保护非原子数据。

## 任务清单

1. 实现一个多线程累加器

```rust
let counter = AtomicUsize::new(0);
// 线程池中 counter.fetch_add(1, Relaxed);
```

验证最终计数正确（只依赖原子性，不依赖顺序）。

2. 构造 Relaxed 失败场景

- 使用 Relaxed 做“标志+数据”模式：
 
  ```rust
  // 错误：Relaxed 可能导致 data 写入对消费者不可见
  data.store(42, Relaxed);
  ready.store(true, Relaxed);
  ```

- 用 loom 或多跑几次（x86 上较难暴露，需用弱内存模型 ARM/Power 模拟）来见证失败。

3. 研究标准库用法

   - 搜索 Rust 源码中 Ordering::Relaxed 的调用，例如 Arc::strong_count 部分实现，理解何时只用 Relaxed 就够了。

## 产出

- 能清楚说出 Relaxed 可以用在哪（独立计数器、无依赖数据的更新），不能用在哪（充当同步标志），并写出会失败的示例。

---

# 第三阶段：Acquire-Release 配对与同步范式（第 3 周）

## 目标

掌握最常见的同步建立方式：Release store + Acquire load，构建“happens-before”关系。

## 任务清单

1. 实现一个简化版 SpinLock

   用 AtomicBool：
   ```rust
  // lock: while self.locked.compare_exchange(false, true, Acquire, Relaxed).is_err() { hint::spin_loop(); }
  // unlock: self.locked.store(false, Release);
   ```
   深入理解：Acquire 获得锁后，能看到 Release 释放前所有内存写入。

2. 单生产者单消费者（SPSC）环形缓冲区

   - 使用 AtomicUsize 的 head/tail 索引。

   - 生产者在写入数据后，以 Release 更新 head；消费者 Acquire 读取 head 后读取数据。

   - 测试数据完整性。

3. 使用 loom 验证正确性

   - 安装 loom，将 Atomic 类型替换为 loom::sync::atomic，给 SpinLock 写一个并发测试，模拟弱内存排序，确认不发生数据竞争。

4. 阅读 “C++ Concurrency in Action” 中对应章节

   - 重点理解 memory model 图，以及如何用 Acquire-Release 建立跨线程顺序。

## 产出

- 自己写的一个无锁 SpinLock 和 SPSC 队列，并通过 loom 验证。

- 能画出成对 Release-Acquire 建立的 happens-before 边。

---

# 第四阶段：SeqCst 与全局总顺序（第 4 周）

## 目标

理解 SeqCst 如何提供“单一全局修改顺序”，而 AcqRel 不能。掌握其代价和适当的替换策略。

## 任务清单

1. 经典 SeqCst 与 AcqRel 差异示例

- 实现常见的 store buffering 或 load buffering 例子，用 SeqCst 保证不出现奇数结果。

- 对比将 SeqCst 改为 AcqRel/Relaxed 导致的松弛行为（用 loom 强制重现）。

2. 阅读 cppreference 的 memory_order 页

- 虽然 C++ 的，但 Rust Ordering 直接对应。重点看 “Sequential-consistency ordering” 和 “Acquire-release ordering” 的区别说明及示例。

3. 性能测试

- 写一个简单的 benchmark：多线程对同一个原子变量执行 fetch_add，分别使用 Relaxed、AcqRel 和 SeqCst，比较吞吐。在 x86 上 SeqCst 和 AcqRel 通常相近（因为硬件本身就提供较强的排序），但理解平台差异。

## 产出

- 能解释为什么 Mutex 的 lock/unlock 通常只需要 AcqRel，而不是强制 SeqCst。

- 有用 loom 展示 SeqCst 和 AcqRel 行为差异的具体测试。

---

# 第五阶段：Fence（屏障）与高级匹配（第 5 周）

## 目标

理解独立 fence (fence(Ordering)) 的作用，以及它与原子操作自身排序的关系，掌握更灵活的同步。

## 任务清单

1. 从原子操作排序到显式 fence

   - 重写 SpinLock：改用 Relaxed 原子操作 + fence(Acquire) / fence(Release)，验证等价性。

   - 注意 fence 的关联规则：Release fence 必须与后续的原子 写 配合，Acquire fence 与读配合。

2. 使用 fence 实现一次性初始化

   - 实现类似 OnceLock 的简化版，用 AtomicU8 状态标记 + fence，避免数据竞争。

3. 阅读 nomicon 的 “Fences” 章节和 Rust 标准库文档

   - 深入理解 fence(SeqCst) 如何提供与其他 SeqCst 操作的全局顺序。

4. 探索 fence 与原子操作顺序的交互

   - 写一个测试：线程 A：store(1, Relaxed); fence(Release); store(2, Relaxed)，线程 B：load(1, Relaxed); fence(Acquire); load(2, Relaxed)，理解可见性。

## 产出

- 能用 fence 重写之前的同步结构，并清晰说出 fence 和带 ordering 的原子操作在语义上的等效变换。

---

# 第六阶段：实战项目 – 构建一个无锁并发容器（第 6 周）

## 目标

整合所有 Ordering 知识，设计并验证一个有一定复杂度的无锁数据结构。

## 选题建议（按难度）

1. 无锁多生产者单消费者队列（MPSC）

   参考 crossbeam 的简化版，使用两个原子索引和 Acquire/Release 操作。

2. 无锁栈（Treiber Stack）

   经典 compare_exchange 循环，使用 AcqRel 或 Acquire/Release 组合。

3. 有界 SPSC channel

   基于环形缓冲区，需仔细放置屏障。

## 实施步骤

- 先用 unsafe 单元 + 原子变量写出核心操作。

- 为每个操作标注排序选择的原因（注释中写清楚：为什么此处用 Acquire，它同步了哪个 Release）。

- 用 loom 进行 CDSCHK（模型检查）验证无数据竞争。

- Benchmark 性能与标准库 Mutex 版本对比。

## 产出

- 一个完整、经过 loom 验证的无锁容器，附带设计文档说明每个 Ordering 的选择逻辑。

---

# 第七阶段：深入硬件与编译器重排（第 7 周及持续）

## 目标

从底层理解内存排序的物理根源，成为“能看穿编译器与 CPU 阴谋”的工程师。

## 任务清单

1. 学习 CPU 缓存一致性与存储缓冲区

   - 阅读 “MESI protocol” 科普和 [Preshing 的博客](https://preshing.com/) 系列，理解 store buffer 为何导致重排。

2. 编译器重排与 Rust 的 nomicon

   - 理解为什么 Rust 的参考（reference）规则和 UnsafeCell 对编译器重排有限制，但原子操作依赖 Ordering 来约束。

3. 观察生成的汇编

   - [Godbolt](https://godbolt.org/) 上编译含不同 Ordering 的代码片段，对比 x86 与 ARM 下的 mfence、dmb 等指令差异。

4. 阅读相关论文或标准摘要

  - C++11 内存模型标准（N2429 等）导读，理解 data race、happens-before、sequentially-consistent 的形式化定义。

## 产出

- 能够解释为什么 x86 上 Acquire-Release 几乎免费，而 ARM 上需要显式屏障指令；并在自己代码中做出平台合理的排序选择。

---

# 推荐资源清单

- Rust 官方

  [std::sync::atomic::Ordering 文档](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
  [Rust Nomicon – Atomics](https://doc.rust-lang.org/nomicon/atomics.html)

- 核心理论

  C++ Concurrency in Action (Anthony Williams) – 第 5 章内存模型和原子操作
  cppreference [memory_order](https://en.cppreference.com/w/cpp/atomic/memory_order)

- 经典博客与图解

  Jeff Preshing 的系列文章（lightweight barrier、happens-before 等）
  [The Rust Performance Book](https://nnethercote.github.io/perf-book/introduction.html) 的原子部分

- 验证工具

  [loom](https://crates.io/crates/loom) – 模型检查，必须动手用
  [criterion](https://crates.io/crates/criterion) – 性能基准

- 源代码学习

  std::sync::Arc, std::sync::Mutex, crossbeam 的无锁队列

---

# 学习方法小贴士

1. 建立 checklist：每次用原子变量时，都问自己“这个操作和哪一个操作建立同步？happens-before 边在哪里？”
2. 先画图后编码：画出线程间的操作时间线和同步边，再映射到 Acquire/Release。
3. 多用 loom：很多问题在 x86 上永远不暴露，必须靠模型检查才能发现。
4. 记录你的困惑：每遇到一个不理解的排序选择，就写一个最小复现代码，通过修改 Ordering 来观察行为变化。

这个计划会让你从“会用原子类型”进化到“能设计正确的无锁算法”。如果中途遇到具体代码或概念上的卡点，随时可以贴出来，一起对着原子变量和内存屏障逐个击破。
