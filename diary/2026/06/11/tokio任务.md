# 一、Tokio 任务的核心定位

Task = Tokio 的绿色线程（用户态轻量级执行单元）

- 由 `tokio::spawn` 创建，本质是一个被运行时托管、可被调度的 `Future`

- 和 OS 线程对比：

  + OS 线程：重、栈大、创建 / 切换昂贵、**抢占式**
  + Tokio Task：极轻、栈小（共享线程栈）、创建开销接近堆分配、**协作式调度**（到 `await` 才让出）

- 一个线程可跑数万～数十万任务，是 Tokio 高并发的核心

一句话：Task = Future + 调度元数据 + 唤醒机制 + 生命周期管理

---

# 二、顶层架构：Runtime → Worker → Task

Tokio 多线程调度器（默认）：

```plaintext
Runtime
├── 线程池：N 个 Worker 线程（默认 = CPU 核数）
├── 每个 Worker 拥有：
│   ├── 本地队列（local queue，无锁、快）
│   ├── next_slot（优化：当前任务唤醒的任务优先执行）
│   └── 偷任务队列（work-stealing，平衡负载）
└── 全局队列（备用，少用）
```

任务在 Worker 间协作 + 抢占混合调度：

- 本地队列优先（cache-friendly）
- 本地空了就偷其他 Worker 的任务（work-stealing）
- 支持任务在 Worker 间迁移（Send）

---

# 三、Task 数据结构（源码级，1.52.3）

## 3.1 核心结构体（简化）

```rust
// tokio::task::Task（Arc 包裹）
struct Task<F: Future> {
    // 任务本体：Future + 状态机
    future: UnsafeCell<F>,

    // 任务状态（关键！后面详细讲）
    state: AtomicUsize,

    // 唤醒器：Waker 绑定到这个任务
    waker: Waker,

    // 调度器句柄：告诉任务怎么入队
    scheduler: Scheduler,

    // 任务 ID、归属 Worker、统计等
    id: u64,
    worker: Option<WorkerId>,
    // ...
}
```

- `Arc<Task<_>>`：任务是共享的，唤醒、调度、取消都要共享引用Tokio
- `UnsafeCell<F>`：`Future` 只被一个线程 `poll`，但需要内部可变性
- `state` 原子变量：整个任务状态机的核心，无锁并发

## 3.2 Task 状态机（最关键）

Tokio 用一个 `AtomicUsize` 编码所有状态，典型值（简化）：

```plaintext
0b0000_0000：IDLE        空闲，待调度
0b0000_0001：RUNNING     正在被 Worker poll
0b0000_0010：NOTIFIED    已唤醒，等待入队
0b0000_0100：COMPLETE    已完成（Ready）
0b0000_1000：CANCELLED   已取消
0b0001_0000：PANICKED    发生 panic
```

状态流转（标准路径）

```plaintext
IDLE →（被调度）→ RUNNING
RUNNING →（poll 返回 Pending）→ NOTIFIED（等待唤醒）
NOTIFIED →（wake）→ 入队 → RUNNING
RUNNING →（poll 返回 Ready）→ COMPLETE → 释放
```

并发安全要点

- 所有状态变更原子操作（CAS）
- 同一时刻只有一个线程能把任务设为 RUNNING
- 避免：多线程同时 poll、重复入队、唤醒竞争

# 四、spawn：任务创建全过程

```rust
let handle: JoinHandle<T> = tokio::spawn(async { ... });
```

## 4.1 步骤拆解

1. async fn → Future 状态机

   编译器把 `async` 函数编译为匿名结构体 + `Future` 实现，每个 `await` 对应状态机的一个节点。
   
2. 包装为 `Task`

    ```rust
    let task = Arc::new(Task {
        future: UnsafeCell::new(future),
        state: AtomicUsize::new(IDLE),
        waker: Waker::from(task_arc.clone()),
        scheduler: runtime.scheduler(),
        // ...
    });
    ```

3. 生成 `JoinHandle`

   - `JoinHandle<T>`：持有 `Arc<Task>`，可 `.await` 结果、检测取消 /panic
   - 分离执行权（Task）和结果所有权（JoinHandle）

4. 首次调度：入队

   - 优先放入当前 Worker 的本地队列尾部
   - 本地队列满 → 随机挑一个 Worker 入队（避免全局锁）

---

# 五、调度与唤醒机制（Tokio 性能核心）

## 5.1 Worker 循环（事件循环）

每个 Worker 线程跑：

```rust
loop {
    // 1. 优先从 next_slot 取（本地唤醒优化）
    if let Some(task) = self.next_slot.take() {
        self.poll_task(task);
        continue;
    }

    // 2. 本地队列取任务
    if let Some(task) = self.local_queue.pop() {
        self.poll_task(task);
        continue;
    }

    // 3. 偷其他 Worker 任务（work-stealing）
    if let Some(task) = self.steal() {
        self.poll_task(task);
        continue;
    }

    // 4. 无任务 → 休眠，等待唤醒/IO事件
    self.wait();
}
```

## 5.2 poll_task：执行任务

```rust
fn poll_task(&self, task: Arc<Task>) {
    // CAS：IDLE → RUNNING，失败则跳过（已被其他线程处理）
    if task.state.compare_exchange(IDLE, RUNNING).is_err() {
        return;
    }

    // 调用 Future::poll
    let cx = Context::new(&task.waker);
    let result = unsafe { (*task.future.get()).poll(Pin::new(&mut *task.future.get()), &mut cx) };

    match result {
        Poll::Ready(output) => {
            // 标记完成，通知 JoinHandle
            task.state.store(COMPLETE, Release);
            task.complete(output);
        }
        Poll::Pending => {
            // 未就绪 → 标记 NOTIFIED，等待唤醒
            task.state.store(NOTIFIED, Release);
        }
    }
}
```

## 5.3 唤醒（Waker）：核心事件驱动

当任务等待的事件就绪（IO、定时器、channel 消息）：

1. 事件源调用 waker.wake()
2. wake 逻辑：
   - CAS 把任务从 NOTIFIED → IDLE
   - 成功则入队（根据唤醒类型选择队列）
   - 内部唤醒（任务 A 唤醒任务 B）：优先进 next_slot（缓存友好）
   - 外部唤醒（IO / 定时器）：进本地队列尾部
   - 自唤醒（yield_now）：强制进本地队列尾部（保证公平）

---

# 六、内存模型与生命周期

## 6.1 `Send` + `'static` 约束

`tokio::spawn` 要求：

```rust
F: Future + Send + 'static
```

- `Send`：任务可在 `Worker` 间迁移，支持 work-stealing
- `'static`：任务生命周期独立，不依赖栈变量（避免悬垂引用）

## 6.2 引用与生命周期管理

- 任务内数据：所有权转移到任务（`move`）
- 共享数据：用 `Arc` + 同步原语（`Mutex`/`RwLock`）
- 禁止：`&'a T` 直接入任务（生命周期不匹配）

## 6.3 任务销毁

- 任务 `COMPLETE`/`CANCELLED` 后：
  1. `Arc<Task>` 计数减一
  2. 计数为 0 → 析构 `Task` → 释放 `Future`、`Waker` 等资源
- `JoinHandle` 被 `drop` 不影响任务执行（分离设计）

---

# 七、任务取消（Cancellation）

Tokio 支持结构化并发 + 取消：

## 7.1 取消触发

- `JoinHandle::abort()`：主动取消
- 父任务取消 → 子任务自动取消（结构化）
- 运行时关闭 → 所有任务取消

## 7.2 取消机制

- 状态设为 `CANCELLED`
- 不再调度，忽略后续唤醒
- `JoinHandle.await` 返回 `Err(Cancelled)`
- 任务内可通过 `tokio::task::is_cancelled()` 检测并清理资源

## 7.3 安全取消

- `Drop` 自动清理资源（锁、句柄、内存）
- 取消不保证立即终止（协作式：必须到 `await` 点才检查）

---

# 八、关键设计亮点（为什么快）

- 无锁本地队列：`Worker` 本地操作几乎无锁，只有跨 `Worker` 偷任务用轻量锁
- `next_slot` 优化：本地唤醒的任务优先执行，提升缓存命中率
- `work-stealing` 负载均衡：空闲 `Worker` 偷忙 `Worker` 任务，避免线程饥饿
- 原子状态机：无锁管理任务状态，高并发下开销极低
- 零成本抽象：`async`/`await` 编译为状态机，无 `runtime` 开销

Tokio 只是把这个状态机工业级实现，加上调度、唤醒、内存安全、取消等能力。
