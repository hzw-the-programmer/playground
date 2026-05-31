Rayon 是 Rust 生态中用于数据并行（data parallelism）的库，能将顺序计算轻松转换为安全的并行计算，且基本不会引入数据竞争。它基于工作窃取（work-stealing）调度，默认使用全局线程池，让开发者以极低的同步开销利用多核 CPU。以下对 Rayon API 进行全面解析，涵盖设计思想、核心模块与使用细节。

---

# 1. 设计思想与核心概念

## 1.1 工作窃取（Work-Stealing）

Rayon 内部维护一个线程池，每个线程有自己的双端任务队列。当线程空闲时，会从其他线程的队列尾部“窃取”任务。这种调度方式天然适合分治算法，能自动平衡负载，避免某些核心忙碌、另一些核心空闲。

## 1.2 全局线程池

Rayon 默认提供一个按 CPU 逻辑核数初始化的全局线程池，通过函数 `rayon::join`、并行迭代器等入口自动使用。无需手动创建，也可按需构建独立的 `ThreadPool`。

## 1.3 安全的数据并行

Rayon 的所有并行抽象均被设计为在 safe Rust 中无法造成数据竞争。它通过 trait 限定和生命周期机制，确保并行任务要么只读共享数据，要么独占可变数据。

---

# 2. 核心 API 模块概览

模块 / Trait	用途
`join`	并发执行两个闭包，经典的分治基础
`scope`	创建作用域，内部可孵化任意数量的并行任务
`ParallelIterator`	将标准迭代器转换为并行迭代器，提供丰富的并行适配器
`par_sort` 等	为 `slice`、`Vec` 等提供的并行排序方法
`ThreadPool`	构建独立的线程池，隔离并发任务
`ThreadPoolBuilder`	配置线程数量、栈大小、线程名等

---

# 3. 基础入口：`join` 与 `scope`

## 3.1 `rayon::join`

```rust
pub fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
where
    A: FnOnce() -> RA + Send,
    B: FnOnce() -> RB + Send,
    RA: Send,
    RB: Send,
```

- 接收两个闭包，可能分别在两个线程上并行执行。

- 当其中一个闭包先完成，其所在线程会尝试窃取其他任务，避免闲置。

- 典型用途：分治算法（如并行快排、归并排序）。

示例：并行分治求和

```rust
use rayon::prelude::*;

fn parallel_sum(slice: &[i32]) -> i32 {
    if slice.len() <= 1000 {
        slice.iter().sum()
    } else {
        let mid = slice.len() / 2;
        let (left, right) = slice.split_at(mid);
        let (sum_l, sum_r) = rayon::join(|| parallel_sum(left), || parallel_sum(right));
        sum_l + sum_r
    }
}
```

这里叶子节点用顺序迭代器避免过小任务的开销。

## 3.2 `rayon::scope`

```rust
pub fn scope<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&Scope<'scope>) -> R + Send,
    R: Send,
```

- 创建一个作用域，传入的 `Scope` 对象可调用 `spawn` 产生子任务。

- 所有在 `scope` 内 `spawn` 的任务都会在该作用域返回前完成，即隐式同步。

- 可以安全借用外部栈变量（生命周期 `'scope` 控制），无须 `'static`。

示例：并行更新多个数组

```rust
use rayon::prelude::*;

fn process(a: &mut [i32], b: &mut [i32]) {
    rayon::scope(|s| {
        s.spawn(|_| a.iter_mut().for_each(|x| *x *= 2));
        s.spawn(|_| b.iter_mut().for_each(|x| *x += 1));
    });
    // 两个 spawn 在此处都已执行完毕
}
```

与直接使用 `std::thread::scope`（Rust 1.63+）相比，Rayon 的 `scope` 任务会被工作窃取调度，适合大量短任务，而标准库的 `scope` 每 `spawn` 产生一个系统线程，更适合少量长任务。

---

# 4. 并行迭代器（ParallelIterator）

这是 Rayon 最常用的能力。任何实现了 `IntoParallelIterator` 的类型都可以调用 `.par_iter()`、`.par_iter_mut()` 或 `.into_par_iter()` 来获得并行迭代器。

## 4.1 创建并行迭代器

方法	迭代类型	说明
`slice.par_iter()`	不可变引用	对切片元素的共享引用并行迭代
`slice.par_iter_mut()`	可变引用	对切片元素的独占引用并行迭代
`vec.into_par_iter()`	所有权转移	消耗集合，并行处理元素
`(0..n).into_par_iter()`	范围	并行遍历整数范围

许多标准库集合实现了 `IntoParallelIterator`，如 `Vec<T>`、`&[T]`、`HashMap`、`BTreeMap`、`Range` 等。

## 4.2 并行迭代器 trait 层次

核心 trait `ParallelIterator` 表示并行可迭代序列，其子 trait `IndexedParallelIterator` 额外提供随机访问、长度已知等优化能力（如切片、范围）。很多适配器根据底层迭代器是否是索引化的，性能特征不同。

## 4.3 常用适配器（Adapter）

几乎所有标准迭代器的方法都有对应的并行版本，且语义相似：

方法	说明
`map`	对每个元素并行应用函数，保持相对顺序
`filter`	并行过滤，保留顺序
`filter_map`	过滤与映射合一
`flat_map`	每个元素映射为并行迭代器后展平
`flatten`	展平嵌套的并行迭代器
`reduce`	无初始值的结合性归约，顺序不定
`fold`	分组折叠，每组分得初始值，最终合并
`for_each`	对每个元素执行副作用操作，无返回值
`collect`	收集到集合，如 `Vec`, `HashMap`
`sum`、`product`	求和、求积（数值类型）
`min`、`max`	最小、最大值（需 `Ord`）
`find_any` / `find_first`	找任意匹配 / 找第一个匹配
`any` / `all`	是否存在 / 全部满足
`count`	计数
`chain`	拼接两个并行迭代器
`zip`	将两个索引化并行迭代器压缩
`enumerate`	添加索引

示例：并行计算素数计数

```rust
use rayon::prelude::*;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    (2..=((n as f64).sqrt() as u64)).all(|d| n % d != 0)
}

let count = (2u64..1_000_000)
    .into_par_iter()
    .filter(|&n| is_prime(n))
    .count();
```

## 4.4 顺序 vs 并行迭代器转换

- 从并行迭代器转为顺序迭代器：`.par_iter().into_seq_iter()`（Rayon 内部会保留某些顺序结果）

- 在并行闭包内部再嵌套顺序迭代器是常见模式（`for_each` 内部使用顺序 `for` 循环）。

---

# 5. `fold` 与 `reduce` 的深入

Rayon 的 `fold` 采用分组 + 聚合模式，避免共享可变状态：

```rust
fn fold<I, ID, F, C>(self, identity: ID, fold_op: F) -> C
where
    I: Send,
    ID: Fn() -> I + Send + Sync,
    F: Fn(I, Self::Item) -> I + Send + Sync,
    C: FoldOp<I>,
```

- `identity`：为每个线程分组提供初始值（通常是闭包，避免共享）。

- `fold_op`：将当前累积值与该组元素结合。

- 最终各组的累积值通过 `C`（通常是 `Sum`、`Product`、自定义 `Reducer`）结合。

`reduce` 是 `fold` 的特殊形式，初始值来自第一个元素（因此必须非空）：

```rust
fn reduce<OP, ID>(self, identity: ID, op: OP) -> Self::Item
where
    OP: Fn(Self::Item, Self::Item) -> Self::Item + Sync + Send,
    ID: Fn() -> Self::Item + Sync + Send,
```

尽管定义中有 `identity`，其实际行为为：如果迭代器为空则调用 `identity`，否则对元素两两归约（无结合顺序保证）。可使用 `reduce_with` 返回 `Option` 避免对空迭代器的 `panic`。

---

# 6. 并行排序与集合操作

Rayon 为切片和 `Vec` 提供了并行排序扩展 trait `ParallelSlice` 和 `ParallelSliceMut`：

```rust
use rayon::prelude::*;

let mut v = vec![5, 1, 8, 2, 7];
v.par_sort();                  // 不稳定排序（快排）
v.par_sort_by(|a, b| ...);    // 自定义比较
v.par_sort_by_key(|k| ...);   // 按键排序
v.par_sort_unstable();        // 明确不稳定排序，但通常 par_sort 自身就是不稳定
```

注意 `par_sort` 是 不稳定的，稳定排序 `par_sort_by_key` 实际上也是不稳定，目前没有稳定并行排序内置实现，需自定义 `par_sort_by` 并保持元素带原始索引。

此外还提供：

- `v.par_chunks_mut(100)`：将可变切片分成固定大小的并行块。

- `v.par_windows(2)`：并行重叠窗口。

---

# 7. 线程池 `ThreadPool` 与配置

## 7.1 构建线程池

```rust
use rayon::ThreadPoolBuilder;

let pool = ThreadPoolBuilder::new()
    .num_threads(8)                // 线程数
    .stack_size(2 * 1024 * 1024)   // 栈大小
    .thread_name(|i| format!("worker-{}", i))
    .build()
    .unwrap();
```

- 若不指定 `num_threads`，默认等于 `available_parallelism()`，即 CPU 逻辑核数。

- 全局线程池首次使用时惰性初始化，也可通过 `ThreadPoolBuilder::build_global()` 手动配置（仅可调用一次，后续调用会 panic）。

## 7.2 在线程池上执行任务

```rust
pool.install(|| {
    // 在此闭包内，rayon::join、par_iter() 等都使用该池
    (0..100).into_par_iter().for_each(|i| { ... });
});
```

- `install` 阻塞当前线程，直到闭包完成，期间当前线程也参与工作窃取。

- 也可用 `pool.spawn(|| ...)` 在池中启动一个异步任务（返回后立即返回，不等待完成），但 `spawn` 必须作用在 `'static` 数据上，因为它可能比当前栈帧存活得更久。通常需配合 `Arc` 等共享所有权。

## 7.3 全局池的访问

- `rayon::current_thread_pool()`：获取当前线程所在的线程池引用（仅在池的 `worker` 线程内调用有效）。

- `rayon::current_num_threads()`：当前池的线程总数。

- `rayon::current_thread_index()`：当前线程在池中的索引（0-based）。

---

# 8. 作用域与生命周期管理

Rayon 对作用域的强调使其能安全借用栈上数据。两种主要方式：

1. `rayon::scope`：显式作用域。

2. 并行迭代器的 `for_each` / `reduce` 等：它们内部自动创建作用域，直接捕获环境变量（非 `static` 也可以），因为并行迭代器执行时会阻塞当前线程直至完成。

例如：

```rust
let mut data = vec![1, 2, 3];
rayon::scope(|s| {
    s.spawn(|_| data.push(4));
    s.spawn(|_| data.push(5));
});
// data 可以被多个 spawn 借用，因为 scope 保证在离开前所有任务完成。
```

但是，不可同时有两个 `spawn` 可变借用同一变量，上面代码中的 `push` 会产生两个可变借用，编译期会报错。正确的做法是用 `par_iter_mut()` 或分块借用（`split_at_mut`），或使用 `Mutex`、`Atomic`。

---

# 9. 高级特性与技巧

## 9.1 分治与 `join` 的效率

- 对于分治算法，当子问题规模较大时 `join` 效率极高；但切勿在极细粒度下调用，任务切换开销会超过并行收益。应设阈值（如 1000～10000 元素）转为顺序执行。

- Rayon 的 `join` 不会在其中一个任务完成时让线程空转，空闲线程会立即窃取任务。

## 9.2 自定义并行迭代器

可以实现 `ParallelIterator` 和 `IndexedParallelIterator` trait 来定义自己的可并行化结构。通常借助 `rayon::iter::plumbing` 下的内部 trait（`UnindexedProducer`, `Producer` 等），但一般情况下组合已有适配器已足够。

## 9.3 错误处理

并行迭代器中 `?` 操作符不能直接使用，因为闭包可能在不同线程执行。常用方式：

- 在闭包内使用 `Result` 收集错误，最后通过 `collect` 或 `try_reduce` 汇总。

- `try_reduce`：类似于 `reduce`，但允许传播错误。

```rust
use rayon::prelude::*;
let result: Result<i32, _> = (0..10)
    .into_par_iter()
    .map(|i| if i == 5 { Err("error") } else { Ok(i) })
    .try_reduce(|| 0, |a, b| Ok(a + b));
```

`try_reduce` 会在发现第一个错误时尽可能取消后续任务。

## 9.4 控制并行度

- 在并行迭代器上调用 `with_min_len`、`with_max_len` 控制拆分粒度，影响负载均衡。

- 调用 `while_some` 提前终止迭代（如 `find_any` 找到元素后，其余线程收到停止信号）。

- 通过 `ThreadPoolBuilder` 设置 `num_threads` 可以在特定工作负载下约束 CPU 占用。

## 9.5 与其他库的集成

- `ndarray`：可通过 `ndarray::parallel` 开启 Rayon 支持。

- `rayon-progress`：为并行迭代器添加进度条。

- `rayon-cond`：提供条件并行，根据元素数量自动选择串行/并行。

---

# 10. 注意事项与最佳实践

1. 避免过小粒度的并行：元素数量少或任务极轻时，并行开销可能大于收益。借助 `with_min_len(1000)` 限制拆分或手动阈值判断。

2. 小心 `for_each` 中的副作用：若多个线程同时写同一变量会导致数据竞争，必须通过 `Atomics`、`Mutex` 或 `par_iter_mut` 的不重叠分片来同步。

- 保持闭包线程安全：传递给 `spawn` 的闭包须为 `Send`；跨线程共享的数据须 `Sync`。编译器的 trait 检查能捕捉多数问题。

- 避免在全局池阻塞：若在 `pool.install` 或并行任务内部使用 `std::thread::sleep` 或阻塞 IO，会占用 worker 线程，降低整体吞吐量。Rayon 适合 CPU 密集型任务；IO 密集型应考虑 async 或额外线程池。

- `collect` 会收集为有序集合：并行 `map` 维持元素相对顺序，`collect` 保证结果顺序与原始一致。若不需要顺序，可用 `collect_into_vec(&mut v)` 填满已有 `Vec`。

- `par_iter_mut` 保证无别名：编译器保证各线程获得不重叠的可变切片，同一索引不会被两个线程同时操作。

- 合理使用 `scope` vs `par_iter`：`scope` 给予更灵活的任务孵化控制，适合异构任务；`par_iter` 适合同构数据并行。

---

# 11. 完整示例：并行统计词频

```rust
use rayon::prelude::*;
use std::collections::HashMap;

fn word_counts(lines: &[String]) -> HashMap<String, usize> {
    lines
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut acc, line| {
                for word in line.split_whitespace() {
                    *acc.entry(word.to_lowercase()).or_insert(0) += 1;
                }
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (k, v) in b {
                    *a.entry(k).or_insert(0) += v;
                }
                a
            },
        )
}
```

这里的 `fold` 为每个线程生成独立 `HashMap`，最后用 `reduce` 合并，全程无共享状态锁，高效且安全。

---

Rayon 将 Rust 的所有权与借用系统延展至并发世界，通过合理的抽象让数据并行如同编写顺序代码一般自然。核心思路是理解 `ParallelIterator` 的适配器语义、善用 `join`/`scope` 进行任务拆解，以及在必要时通过 `ThreadPool` 隔离负载。结合这些 API，便可写出既安全又充分利用多核性能的 Rust 程序。
