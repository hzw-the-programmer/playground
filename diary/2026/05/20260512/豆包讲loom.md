下面我从原理、核心能力、用法、实战、局限、最佳实践六个维度，把 Rust Loom 讲透，直接能上手写测试。

---

# 一、Loom 是什么？

Loom = 并发模型检查器（model checker），专门给 Rust 无锁 / 并发代码做穷尽式、确定性测试。

## 解决的痛点

- 并发 bug（数据竞争、内存序错误）随机出现、难复现
- 靠 “多跑几次” 没用，覆盖率极低
- Loom 模拟调度 + 模拟内存模型，遍历所有合法执行路径，逼出隐藏 bug

## 核心定位

- 测试对象：atomic、无锁队列、Mutex、Condvar、线程池 等
- 理论基础：C11 内存模型（Rust 继承它）
- 作者：Tokio 团队（carllerche）

---

# 二、工作原理（极简但关键）

1. 替换标准库类型

测试时不用 std::sync::atomic::*，而是用 loom::sync::atomic::*：

```rust
// 生产代码
use std::sync::atomic::{AtomicBool, Ordering};

// Loom 测试代码
use loom::sync::atomic::{AtomicBool, Ordering};
use loom::thread;
```

Loom 的类型会拦截所有 load/store/RMW 操作，记录状态，控制调度。

2. 模拟调度：穷尽线程交错

- Loom 不依赖 OS 调度，自己控制线程切换点
- 在每一次原子操作、线程创建 / 销毁时，尝试所有可能的调度顺序
- 用 ** 状态压缩（state reduction）** 避免路径爆炸，能跑但慢

3. 模拟内存模型：允许 “合法乱序”

严格按 C11 内存模型，允许：
- Relaxed：重排、可见性延迟
- Release/Acquire：保证 “写后读” 可见性
- SeqCst：全局顺序（部分支持）

Loom 会主动触发真实硬件可能出现的乱序行为，比如：
- 先读到 flag=true，后看到 data=0（Relaxed 错误）
- 两个线程互相看到对方的旧值（load buffering）

---

# 三、快速上手（5 分钟跑通）

1. 加依赖

```toml
# Cargo.toml
[dev-dependencies]
loom = "0.7"  # 用最新版
```

2. 写第一个 Loom 测试（必现 Relaxed bug）

```rust
// tests/loom_demo.rs
use loom::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use loom::thread;
use std::sync::Arc;

#[test]
fn relaxed_can_cause_wrong_value() {
    loom::model(|| {
        let ready = Arc::new(AtomicBool::new(false));
        let data = Arc::new(AtomicI32::new(0));

        let writer = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                data.store(42, Ordering::Relaxed);
                ready.store(true, Ordering::Relaxed);
            })
        };

        let reader = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                // 只检查一次，不循环
                if ready.load(Ordering::Relaxed) {
                    let d = data.load(Ordering::Relaxed);
                    assert_eq!(d, 42, "Reader saw an invalid value!");
                }
            })
        };

        writer.join().unwrap();
        reader.join().unwrap();
    });
}
```

3. 运行测试

```bash
# 必须加 --cfg loom，否则用 std 类型
RUSTFLAGS="--cfg loom" cargo test --test loom_demo --release
```

4. 结果

测试必失败，并输出：

```text
thread 'test::relaxed_can_cause_wrong_value' (26608) panicked at examples\ordering\src\test.rs:36:21:
assertion `left == right` failed: Reader saw an invalid value!
  left: 0
 right: 42
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这就是 Loom 帮你抓到的内存序错误。

---

# 四、核心 API 详解

1. loom::model(f)：入口

- 自动遍历所有线程调度 + 内存序组合
- 内部会多次执行 f，每次不同路径
- 失败时打印完整执行轨迹，可复现

2. 支持的类型（替换 std）

std	loom
std::sync::atomic::Atomic*	loom::sync::atomic::Atomic*
std::sync::Mutex	loom::sync::Mutex
std::sync::Condvar	loom::sync::Condvar
std::thread::spawn	loom::thread::spawn
std::sync::Arc	loom::sync::Arc

3. Ordering 支持

- ✅ Relaxed：完全支持，主动乱序
- ✅ Release / Acquire：完全支持，严格可见性
- ✅ AcqRel：支持（RMW 操作）
- ⚠️ SeqCst：部分支持，不保证全局顺序

4. 配置 Builder（控制复杂度）

```rust
loom::model::Builder::new()
    .max_threads(4)          // 最大线程数
    .max_branches(1000)      // 最大分支数
    .preemption_bound(5)      // 线程切换次数上限
    .log(true)                // 输出详细日志
    .run(|| {
        // 测试代码
    });
```

避免路径爆炸，缩短测试时间。

---

# 五、实战：用 Loom 验证 Release-Acquire 正确

```rust
#[test]
fn release_acquire_is_correct() {
    loom::model(|| {
        let ready = Arc::new(AtomicBool::new(false));
        let data = Arc::new(AtomicI32::new(0));

        let writer = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                data.store(42, Ordering::Relaxed);
                ready.store(true, Ordering::Release);
            })
        };

        let reader = {
            let ready = ready.clone();
            let data = data.clone();
            thread::spawn(move || {
                if ready.load(Ordering::Acquire) {
                    let d = data.load(Ordering::Relaxed);
                    assert_eq!(d, 42);
                }
            })
        };

        writer.join().unwrap();
        reader.join().unwrap();
    });
}
```

测试永远通过，证明 Release-Acquire 能阻止乱序。

---

# 六、Loom 的局限（必须知道）

1. 不支持完整 SeqCst：多线程 SeqCst 操作可能漏检
2. 不支持 load buffering 全场景：少数 C11 允许的行为 Loom 不模拟
3. 运行慢：路径多，复杂测试可能跑几分钟
4. 不能测异步代码：仅支持原生线程，不支持 Tokio/async-std
5. 必须替换类型：所有并发原语都要用 loom 版，不能混 std

---

# 七、最佳实践

1. 测试无锁代码必用 Loom

- 所有 atomic 操作、无锁队列、内存屏障逻辑，必须写 Loom 测试
- 覆盖 Relaxed/Release/Acquire/AcqRel 所有组合

2. 生产 / 测试类型分离（cfg 控制）

```rust
#[cfg(loom)]
use loom::sync::atomic::{AtomicBool, Ordering};
#[cfg(not(loom))]
use std::sync::atomic::{AtomicBool, Ordering};
```

一套代码，生产用 std，测试用 loom。

3. 从小用例开始

- 先测 2 线程、2 个原子变量的简单场景
- 逐步增加复杂度，避免一开始就路径爆炸

4. 结合 miri

- Loom：测内存序 + 线程交错
- Miri：测数据竞争 + 未定义行为
- 两者互补，覆盖 99% 并发 bug

---

# 八、总结（一句话记牢）

Loom 是并发安全的 “压力测试器”，模拟所有合法线程调度与内存序乱序，帮你在开发阶段抓出 Relaxed 滥用、Release-Acquire 配对错误等隐蔽 bug，是 Rust 无锁开发的必备工具。
