CAS = Compare-And-Swap，比较并交换，是无锁并发的核心原子操作。

# 一、字面含义（三步动作，硬件原子执行，中途不会被打断）

CAS 接收 3 个参数：内存地址、预期旧值、新值

1. 比较：读取目标内存当前值，判断是否 = 预期旧值
2. 交换
  + 相等：把内存值改成新值，返回「成功」
  + 不相等：什么都不做，返回「失败」
3. 整个过程由 CPU 硬件保证原子性，多线程并发下不会被插队。

---

# 二、基础伪代码（逻辑版）

```rust
// 伪代码：原子 CAS
fn cas(addr: &mut usize, expected: usize, new: usize) -> bool {
    if *addr == expected {
        *addr = new;
        true // 交换成功
    } else {
        false // 有人改了，交换失败
    }
}
```

重点：普通 if + 赋值 是两步，会被线程打断；CPU 原生 CAS 是一步原子指令。

---

# 三、结合 Tokio Task 场景理解（最实用）

Tokio 用 `AtomicUsize` 存任务状态，靠 CAS 保证：同一时刻只有一个线程能执行这个任务。

场景：抢任务执行权（IDLE → RUNNING）

任务初始状态：IDLE(0)
多个 Worker 线程同时看到这个空闲任务，都想 poll 它：

```rust
// Tokio 真实逻辑简化
let old = IDLE;
let new = RUNNING;

// CAS：预期当前是 IDLE，就改成 RUNNING
match state.compare_exchange(old, new) {
    Ok(_) => {
        // 成功：我抢到了，开始 poll 任务
        poll_task();
    }
    Err(actual) => {
        // 失败：状态已经不是 IDLE
        // 说明别的线程已经抢先把它改成 RUNNING，直接跳过
    }
}
```

举个并发例子

1. 线程 A、线程 B 同时发现任务是 IDLE
2. 线程 A 先执行 CAS：IDLE → RUNNING，成功，开始执行任务
3. 线程 B 再执行 CAS：发现当前值已经是 RUNNING ≠ 预期 IDLE，失败，直接放弃

👉 效果：天然实现互斥，不用加传统锁（Mutex），性能远高于互斥锁。

---

# 四、CAS 常见用法 & 概念

## 1. 自旋（Spin）

CAS 失败时不休眠，循环重试，叫自旋锁：

```rust
// 自旋等待直到抢到状态
loop {
    match state.compare_exchange(IDLE, RUNNING) {
        Ok(_) => break,
        Err(_) => continue,
    }
}
```

Tokio 部分短竞争场景会用轻量自旋。

## 2. ABA 问题（简单提）

经典并发坑：

- 线程 1 读取值 = A，准备 CAS
- 线程 2 先把 A→B，再改回 A
- 线程 1 看到还是 A，以为没人改动，继续交换 → 逻辑出错

Tokio 任务状态机通过状态不可逆设计（一旦进入 COMPLETE/CANCELLED 就不再回退），规避了 ABA。

# 五、一句话总结

CAS 是 CPU 提供的「原子比较 + 赋值」指令，用来在不加锁的前提下，实现多线程安全地修改共享状态，是 Tokio 高性能无锁调度的基石。
