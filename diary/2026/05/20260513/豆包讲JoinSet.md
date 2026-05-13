详解 tokio::task::JoinSet

---

`JoinSet` 是 Tokio 官方提供的**动态任务集合管理器**，用来替代手写 `Vec<JoinHandle<T>>`，核心价值是：**按完成顺序收结果、动态增删任务、自动取消、内置安全**。下面从定位、源码结构、API、用法、与 Vec 对比、陷阱、最佳实践逐一讲透。

---

# 一、定位与核心价值

1. **解决的痛点**

- 手动用 `Vec<JoinHandle<T>>`：只能**按创建顺序等待**，先完成的任务结果必须等前面的，浪费时间。
- 动态任务：运行中不断 spawn 新任务、取消旧任务，手动管理复杂。
- 错误处理：一个任务 panic，需要手动遍历 cancel 剩余任务。
- 资源泄漏：忘记 await 的 JoinHandle 会导致任务残留。

2. **核心特性**

- ✅ **完成顺序返回**：`join_next().await` 哪个先完先返回谁。
- ✅ **动态增删**：随时 `spawn`、随时 `abort`、随时 `join_next`。
- ✅ **自动取消**：`JoinSet` drop 时，所有未完成任务**自动 abort**。
- ✅ **取消安全**：`join_next` 可放在 `select!` 里，分支取消不丢任务。
- ✅ **统一错误**：`JoinError` 统一处理 panic/cancel。

3. **适用场景**

- 批量并发（爬虫、文件处理、API 调用）
- 动态任务（连接池、消息处理、流式任务）
- 限并发（配合信号量）
- 任务组生命周期绑定（一个 JoinSet 对应一组任务）

---

# 二、源码结构（简化）

```rust
pub struct JoinSet<T> {
    inner: IdleNotifiedSet<JoinHandle<T>>,
}

impl<T> JoinSet<T> {
    pub fn new() -> Self;
    pub fn spawn<F>(&mut self, f: F) -> AbortHandle;
    pub async fn join_next(&mut self) -> Option<Result<T, JoinError>>;
    pub async fn join_all(self) -> Vec<T>;
    pub fn abort_all(&mut self);
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

// Drop 时自动 abort 所有任务
impl<T> Drop for JoinSet<T> {
    fn drop(&mut self) {
        self.inner.drain(|jh| jh.abort());
    }
}
```

核心是 `IdleNotifiedSet`：管理空闲 / 活跃任务，任务完成时唤醒等待者。

---

# 三、核心 API 详解

1. **创建**

```rust
let mut set = JoinSet::new();
```

2. **生成任务（spawn）**

```rust
pub fn spawn<F>(&mut self, task: F) -> AbortHandle
where
    F: Future<Output = T> + Send + 'static,
    T: Send,
```

- 立即提交到 runtime 执行，**不用 await 就跑**。
- 返回 `AbortHandle`：可随时取消单个任务。
- 约束：`Future` 必须 `Send + 'static`（Tokio 任务要求）。

3. **等待下一个完成（join_next）**

```rust
pub async fn join_next(&mut self) -> Option<Result<T, JoinError>>;
```

- 返回 `Some(Ok(val))`：任务正常完成。
- 返回 `Some(Err(JoinError))`：任务 panic 或被取消。
- 返回 `None`：集合空了。
- **取消安全**：`select!` 中取消此分支，任务仍在集合里。

4. **等待所有（join_all）**

```rust
pub async fn join_all(self) -> Vec<T>;
```

- 消费 `self`，等待所有任务完成，返回完成顺序的结果。
- 任意任务 panic：**立即传播 panic**，剩余任务自动取消。

5. **取消所有（abort_all）**

```rust
pub fn abort_all(&mut self);
```

- 立刻取消所有未完成任务，不等待。

6. **工具方法**

- `len()`：当前任务数（含运行中 + 已完成未取）
- `is_empty()`：是否无任务

---

# 四、完整可运行示例

## 示例 1：基础用法（完成顺序收集）

```rust
use tokio::task::JoinSet;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    //  spawn 3个任务，休眠时间不同
    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        3
    });
    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        1
    });
    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        2
    });

    // 按完成顺序收结果：1 → 2 → 3
    while let Some(res) = set.join_next().await {
        println!("完成: {}", res.unwrap());
    }
}
```

输出：

```text
完成: 1
完成: 2
完成: 3
```

## 示例 2：动态添加任务

```rust
use tokio::task::JoinSet;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    // 初始任务
    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("初始任务完成");
    });

    // 动态添加：每完成一个就加一个新的
    while let Some(res) = set.join_next().await {
        res.unwrap();
        // if set.len() < 5 {
            set.spawn(async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("动态任务完成");
            });
        // }
    }
}
```

## 示例 3：取消单个任务

```rust
use tokio::task::JoinSet;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    let abort_handle = set.spawn(async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("长任务完成");
    });

    // 2秒后取消
    tokio::time::sleep(Duration::from_secs(2)).await;
    abort_handle.abort();

    // 收结果：被取消
    let res = set.join_next().await.unwrap();
    assert!(res.unwrap_err().is_cancelled());
}
```

---

# 五、与 Vec<JoinHandle<T>> 对比

特性	`JoinSet<T>`	`Vec<JoinHandle<T>>`
返回顺序	完成顺序	创建顺序
动态增删	原生支持	需手动 push/pop
自动取消	Drop 时全取消	需手动遍历 cancel
取消安全	✅ `select!` 安全	❌ 取消会丢任务
错误处理	统一 `JoinError`	需手动处理每个 handle
代码简洁度	高（5 行搞定）	低（循环 + 状态管理）

结论：**99% 场景优先用 JoinSet**，仅需严格顺序时用 Vec。

---

# 六、关键陷阱与注意事项

1. **`'static` 约束**

- `spawn` 的 future 必须 `Send + 'static`，**不能直接用栈引用**。
- 解决：用 `Arc` 克隆，或把数据 move 进任务。

2. **`join_all` 的 panic 传播**

- `join_all` 遇到第一个 panic 就**立即抛出**，剩余任务取消。
- 解决：用 `join_next` 循环，逐个处理错误，不中断整体。

3. **不要混合 `JoinSet` 与手动 `JoinHandle`**

- `JoinSet` 接管任务生命周期，手动 await 其 `JoinHandle` 会导致**双重使用**，触发 panic。

4. **限并发必须配合信号量**

- `JoinSet` 本身**不限并发**，spawn 多少跑多少。
- 限并发：用 `tokio::sync::Semaphore`，spawn 前获取许可。

5. **Drop 时自动取消**

- 离开作用域时 `JoinSet` drop，所有未完成任务**强制 abort**。
- 需等待任务优雅结束：必须 `join_next` 到空或调用 `join_all`。

---

# 七、最佳实践

1. **批量任务模板（推荐）**

```rust
use tokio::task::JoinSet;

async fn batch_process<T, F, Fut>(items: Vec<T>, f: F) -> Vec<Result<Fut::Output, tokio::task::JoinError>>
where
    T: Send + 'static,
    F: Fn(T) -> Fut + Send + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    let mut set = JoinSet::new();
    for item in items {
        set.spawn(f(item));
    }
    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res);
    }
    results
}
```

2. **限并发模板**

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

async fn limited_concurrent<T, F, Fut>(
    items: Vec<T>,
    max_concurrent: usize,
    f: F,
) -> Vec<Fut::Output>
where
    T: Send + 'static,
    F: Fn(T) -> Fut + Send + 'static + Clone,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    let sem = Arc::new(Semaphore::new(max_concurrent));
    let mut set = JoinSet::new();

    for item in items {
        let sem = sem.clone();
        let f = f.clone();
        set.spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            f(item).await
        });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res.unwrap());
    }
    results
}
```

---

# 八、总结（一句话）

**`JoinSet` 是 Tokio 异步并发的 “任务管家”，用完成顺序收结果、动态管理生命周期、自动处理取消与错误，让并发代码简洁、安全、高效，是批量 / 动态并发场景的首选。**