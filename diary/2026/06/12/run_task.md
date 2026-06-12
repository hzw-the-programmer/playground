# 一、函数整体概览

函数签名

```rust
fn run_task(&self, task: Notified, mut core: Box<Core>) -> RunResult
```

- 参数：
  + `self`：当前 `Worker` 线程的执行上下文
  + `task: Notified`：被唤醒、标记为可调度的待执行任务
  + `core: Box<Core>`：`Worker` 的核心状态（包含运行队列、统计信息、LIFO 槽位、I/O 驱动状态等）

- 返回值：`Result<Box<Core>, ()>`，成功返回更新后的 `Worker` 核心状态，失败表示核心状态被其他 `Worker` 线程偷走（工作窃取机制）

核心设计目标

1. 最大化 CPU 缓存命中率（LIFO 槽位优化）
2. 防止单个任务垄断线程（协作调度预算）
3. 避免任务饥饿（LIFO 限流 + 工作窃取负载均衡）
4. 低延迟 I/O 处理（驱动快速移交优化）

---

# 二、逐段代码解析

## 1. 不稳定特性：任务元数据获取

```rust
#[cfg(tokio_unstable)]
let task_meta = task.task_meta();
```

- 仅当开启 `tokio_unstable` 不稳定特性时编译，用于获取任务的元数据（任务 ID、调度信息、来源等）
- 用途：供后续任务钩子（task hooks）使用，支持自定义监控、APM 追踪、调试等扩展能力，属于 Tokio 的非稳定实验特性。

## 2. 任务所有权校验

```rust
let task = self.worker.handle.shared.owned.assert_owner(task);
```

Tokio 工作窃取调度的安全保障：

- 每个 Worker 维护自己的任务所有权集合，这里断言当前 Worker 是该任务的合法所有者
- 防止任务被错误的线程执行，避免数据竞争和调度逻辑混乱，是内存安全的关键校验。

## 3. 退出搜索状态，开放工作窃取

```rust
let notified_parked_worker = core.transition_from_searching(&self.worker);
```

工作窃取机制的核心状态切换：

- Worker 空闲寻找任务时会进入 searching 状态，此时其他 Worker 不能从它的队列窃取任务
- 当前 Worker 已经拿到待执行任务，主动退出搜索状态，允许其他空闲 Worker 从自己的队列偷任务，实现全局负载均衡
- 返回值 notified_parked_worker：表示是否已经唤醒了一个处于停车（parked）状态的空闲 Worker。

## 4. 不稳定优化：I/O 驱动快速移交

```rust
if cfg!(tokio_unstable)
    && core.enable_eager_driver_handoff
    && core.had_driver == park::HadDriver::Yes
    && !notified_parked_worker
{
    core.had_driver = park::HadDriver::No;
    self.worker.handle.notify_parked_local();
}
```

Tokio 针对 I/O 延迟的实验性优化：

- 背景：每个 Worker 可以持有 I/O 驱动（epoll/kqueue 实例），负责处理 I/O 事件。如果当前 Worker 要执行 CPU 密集任务，I/O 事件可能得不到及时处理
- 逻辑：如果开启了快速移交开关、当前 Worker 持有 I/O 驱动，且还没唤醒其他空闲 Worker，就主动唤醒一个本地停车的 Worker，让它接手 I/O 驱动
- 目的：避免 I/O 事件被 CPU 任务阻塞，降低长尾延迟，仅在不稳定特性下生效。

## 5. 调试校验：LIFO 状态正确性

```rust
self.assert_lifo_enabled_is_correct(&core);
```

- 仅 Debug 模式生效的断言，校验 LIFO 槽位的启用状态是否符合预期
- 用于捕获调度逻辑的 BUG，保障 LIFO 优化的正确性。

## 6. 统计：开始任务轮询计时

```rust
core.stats.start_poll();
```

- 启动本次任务执行的性能统计，记录轮询开始时间、任务计数等指标
- 用于调度器的性能监控、负载评估和调优。

## 7. 绑定 Core 到线程本地上下文

```rust
*self.core.borrow_mut() = Some(core);
```

- `self.core` 是线程本地存储的 `RefCell<Option<Box<Core>>>`，把当前 `Worker` 的核心状态存入线程本地
- 作用：任务执行过程中（比如 `spawn` 新任务、唤醒其他任务、更新统计），可以通过线程上下文快速访问 `Core`，无需传参。

## 8. 协作调度：预算包裹执行上下文

```rust
coop::budget(|| {
    // 所有任务执行都在这个闭包内，受预算控制
})
```

Tokio 协作调度的核心机制：

- 预算原理：每个任务默认分配 128 次异步操作预算，每次 `.await` 异步操作都会消耗 1 点预算
- 作用：预算耗尽后任务必须主动让出 CPU，防止单个长任务垄断 Worker 线程，保障调度公平性，避免其他任务饥饿
- 闭包内的所有执行（主任务 + LIFO 任务）都受同一个预算控制。

## 9. 执行主任务（含钩子回调）

```rust
#[cfg(tokio_unstable)]
self.worker.handle.task_hooks.poll_start_callback(&task_meta);

task.run(); // 真正执行任务的 Future::poll

#[cfg(tokio_unstable)]
self.worker.handle.task_hooks.poll_stop_callback(&task_meta);
```

- 开启不稳定特性时，在任务轮询前后触发自定义钩子，支持任务级别的监控、埋点、调试
- `task.run()`：核心逻辑，调用任务 `Future` 的 `poll` 方法，推进异步任务的执行进度（比如处理 I/O 事件、执行异步逻辑）。

## 10. LIFO 槽位任务循环执行（核心优化）

```rust
let mut lifo_polls = 0;
loop {
    // 1. 取出 Core，检查是否被其他 Worker 偷走
    let mut core = match self.core.borrow_mut().take() {
        Some(core) => core,
        None => return Err(()), // Core 被窃取，终止执行
    };

    // 2. 尝试从 LIFO 槽位取任务
    let task = match core.lifo_slot.take() {
        Some(task) => task,
        None => {
            // LIFO 无任务，结束执行
            self.reset_lifo_enabled(&mut core);
            core.stats.end_poll();
            return Ok(core);
        }
    };

    // 3. 预算耗尽：把 LIFO 任务退回普通队列
    if !coop::has_budget_remaining() {
        core.stats.end_poll();
        core.run_queue.push_back_or_overflow(
            task,
            &*self.worker.handle,
            &mut core.stats,
        );
        return Ok(core);
    }

    // 4. LIFO 限流：防止乒乓任务饿死其他任务
    lifo_polls += 1;
    super::counters::inc_lifo_schedules();

    if lifo_polls >= MAX_LIFO_POLLS_PER_TICK { // 默认 31 次
        core.lifo_enabled = false;
        super::counters::inc_lifo_capped();
    }

    // 5. 执行 LIFO 任务，循环继续
    *self.core.borrow_mut() = Some(core);
    let task = self.worker.handle.shared.owned.assert_owner(task);
    // （同主任务：钩子 + 执行逻辑）
    task.run();
}
```

这是 Tokio 最具特色的性能优化，详细拆解：

① LIFO 槽位的设计原理

- 什么是 LIFO 槽位：每个 Worker 有一个单元素的优先级槽位，优先级远高于普通运行队列
- 触发场景：任务 A 唤醒任务 B（比如 channel 发消息、spawn 子任务后立即 await）时，B 会被优先放入 LIFO 槽位，而不是普通队列
- 性能收益：任务 A 刚执行完，它修改的数据还在 CPU L1/L2 缓存中，任务 B 立即执行能100% 命中缓存，避免内存访问开销，典型场景下性能提升可达 30% 以上。

② Core 窃取检查

每次循环都要重新从线程本地取出 Core：如果 Core 为 None，说明当前 Worker 的核心状态被其他空闲 Worker 偷走了（工作窃取机制），直接终止执行，让窃取的 Worker 继续处理。

③ LIFO 限流防饥饿

- 问题：如果两个任务互相唤醒（A 唤醒 B，B 唤醒 A，循环往复），它们会永远占用 LIFO 槽位，导致普通队列的任务永远得不到执行（饥饿）
- 解决方案：单次 tick 内 LIFO 任务最多执行 31 次（MAX_LIFO_POLLS_PER_TICK），超过限制后禁用 LIFO 槽位，强制后续任务走普通运行队列，保障公平性。

# 三、核心设计思想总结

这个函数浓缩了 Tokio 调度器的四大核心设计哲学：

- 性能优先：缓存局部性最大化

  通过 LIFO 槽位优先执行刚被唤醒的任务，极致利用 CPU 缓存，减少内存访问开销。

- 公平性保障：防饥饿设计

  协作预算 + LIFO 限流 + 工作窃取，三重机制避免任务饿死，保证调度公平。

- 负载均衡：工作窃取机制

  退出搜索状态允许其他 Worker 偷任务，自动平衡多线程负载，避免部分线程空闲、部分线程过载。

- 可扩展：分层特性设计

  稳定核心逻辑 + 不稳定扩展特性（钩子、I/O 移交），兼顾生产环境稳定性和未来优化空间。
