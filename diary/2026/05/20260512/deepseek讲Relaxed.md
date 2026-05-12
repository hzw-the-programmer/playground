用一个具体例子说明 乱用Relaxed会造成的错误

---

用一个经典的数据就绪标志（ready flag）的例子，可以很直观地展示全部用 Relaxed 会如何出问题。

# 场景描述

一个线程负责准备数据，然后通知另一个线程数据已经就绪；另一个线程不断检查通知，一旦看到通知就立即使用数据。

# 错误实现（全程 Relaxed）

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static READY: AtomicBool = AtomicBool::new(false);
static mut DATA: i32 = 0; // 模拟共享数据，实际中可能是一块缓冲区

fn main() {
    let writer = thread::spawn(|| {
        // 1. 准备数据
        unsafe { DATA = 42; }
        // 2. 通知读取者
        READY.store(true, Ordering::Relaxed);
    });

    let reader = thread::spawn(|| {
        // 3. 自旋等待就绪标志
        while !READY.load(Ordering::Relaxed) {
            // 忙等
        }
        // 4. 一旦看到标志，直接使用数据
        let d = unsafe { DATA };
        println!("读取到的数据: {}", d);
        // 期望 d == 42，但 Relaxed 下可能看到 0 或其他结果
    });

    writer.join().unwrap();
    reader.join().unwrap();
}
```

# 会导致什么错误？

对于读取线程，可能看到 d 的值为 0（或任何非 42 的值），甚至触发未定义行为。

原因：Ordering::Relaxed 只保证单个原子变量的原子性（对该变量的读/写不会出现中间状态），但不提供任何跨变量的执行顺序或可见性保证。编译器或 CPU 可以随意重排没有直接依赖关系的指令：

- 写线程可能发生的重排：编译器／CPU 可能把 READY.store(true, Relaxed) 提前到 DATA = 42 之前执行。这样读取线程看到 READY == true 时，数据还没写入。
- 读线程可能发生的重排：编译器／CPU 可能把 DATA 的读 提前 到 while !READY.load(Relaxed) 之前完成。也就是说，读线程可能在循环还没退出时就已经读取了一个旧的 DATA 值缓存起来，随后即便标志变为 true，使用的仍是陈旧的缓存值。
- 没有 happens-before 关系：在内存模型上，写线程的 DATA = 42 与读线程的 unsafe { DATA } 之间没有建立任何先行发生关系（happens-before），因此二者对同一非原子变量的并发访问构成了数据竞争，在 Rust 中属于未定义行为。

在 x86 这类强内存模型的平台上，这段代码很可能“碰巧”工作正常，但在 ARM、PowerPC 等弱内存模型平台上，或者启用更激进的编译器优化后，问题就会暴露出来。不依赖平台恰好给予的保障，正是使用 Relaxed 时最容易犯的错误。

# 正确做法：用 Release-Acquire 建立次序

让标志的写和读形成一对同步操作：

```rust
// 写线程
READY.store(true, Ordering::Release);   // 保证在 store 之前的所有写入对后续 acquire 可见

// 读线程
while !READY.load(Ordering::Acquire) {} // 看到 Release 写入后，后续读保证能看到对方之前所有写入
let d = unsafe { DATA };                // 现在必定能看到 DATA = 42
```

- Release 保证：写线程中在 store 之前的所有内存写入（包括 DATA = 42）都不会被重排到 store 之后。当另一个线程通过 Acquire 读到了这次写入（或更晚的写入），就建立了 happens-before 关系。
- Acquire 保证：读线程中在 load 之后的所有操作（包括读 DATA）都不会被重排到 load 之前。因此读到的 DATA 一定是写入线程在释放点之前写入的值。

# 总结教训

- Relaxed 只适用于不与其他数据存在先后依赖的场景，例如单纯的全局访问计数器（只关心最终数量，不在乎与其他变量的时序）。
- 只要一个变量的写入是另一个线程访问其他共享数据的“开关”，就必须使用 Release/Acquire（或更强的定序）来保证因果顺序。
- 用 Relaxed 偷性能却破坏了正确性，是并发编程中代价极高的一类错误。先用正确的方式实现，再用基准测试和正确性模型（如 Loom）验证是否有下调到更弱定序的空间。
