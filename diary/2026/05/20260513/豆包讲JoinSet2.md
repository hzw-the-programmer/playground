# Tokio JoinSet 极简常用模板 + 错误处理清单（可直接复制复用）

## 前置统一依赖

```toml
# Cargo.toml
tokio = { version = "1.0", features = ["full"] }
```

---

## 模板 1：基础批量并发（按完成顺序收结果）

```rust
use tokio::task::JoinSet;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    // 批量抛任务
    for i in 1..=5 {
        set.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100 * (6 - i))).await;
            i
        });
    }

    // 逐个按完成顺序收割
    while let Some(res) = set.join_next().await {
        match res {
            Ok(val) => println!("任务完成: {}", val),
            Err(e) => {
                if e.is_cancelled() {
                    eprintln!("任务被取消");
                } else if e.is_panic() {
                    eprintln!("任务panic: {:?}", e);
                }
            }
        }
    }
}
```

---

## 模板 2：限制最大并发（Semaphore + JoinSet）

最常用业务模板，控制同时跑的任务数

```rust
use tokio::task::JoinSet;
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    let sem = Arc::new(Semaphore::new(3)); // 最大并发3

    for i in 1..=10 {
        let sem = sem.clone();
        set.spawn(async move {
            // 获取令牌，自动限流
            let _permit = sem.acquire().await.unwrap();

            tokio::time::sleep(Duration::from_millis(200)).await;
            println!("处理完成: {}", i);
            i
        });
    }

    // 等待全部完成，忽略错误可 unwrap，严谨用match
    while let Some(res) = set.join_next().await {
        let _ = res;
    }
}
```

---

## 模板 3：主动取消单个任务

```rust
use tokio::task::JoinSet;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    //  spawn 拿到取消句柄
    let handle = set.spawn(async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        "长任务结果"
    });

    // 2秒后取消
    tokio::time::sleep(Duration::from_secs(2)).await;
    handle.abort();

    // 检测是否被取消
    let res = set.join_next().await.unwrap();
    assert!(res.unwrap_err().is_cancelled());
}
```

---

## 模板 4：一次性等待所有任务（join_all）

```rust
use tokio::task::JoinSet;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    set.spawn(async { tokio::time::sleep(Duration::from_secs(1)).await; 1 });
    set.spawn(async { tokio::time::sleep(Duration::from_secs(2)).await; 2 });

    // 消费self，等待全部完成，返回完成顺序列表
    let all = set.join_all().await;
    println!("所有结果: {:?}", all);
}
```

注意：**任意任务 panic，join_all 直接抛 panic**

---

## 模板 5：select! 配合 JoinSet（超时 / 信号退出）

```rust
use tokio::task::JoinSet;
use tokio::select;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    set.spawn(async { tokio::time::sleep(Duration::from_secs(10)).await; });

    select! {
        _ = tokio::time::sleep(Duration::from_secs(3)) => {
            println!("超时，取消所有任务");
            set.abort_all();
        }
        _ = set.join_next() => {}
    }
}
```

---

## JoinError 错误处理速查（必背）

```rust
while let Some(res) = set.join_next().await {
    match res {
        Ok(v) => { /* 正常完成 */ }
        Err(e) => {
            if e.is_cancelled() {
                // 任务被主动 abort / JoinSet 被drop
            } else if e.is_panic() {
                // 任务内部panic
            }
            // e.into_panic() 可抓取panic信息
        }
    }
}
```

---

## 核心禁忌 & 最佳实践口诀

- JoinSet Drop 自动取消所有未完成任务，不用手动遍历取消。
- 要限流 必配 Semaphore，JoinSet 本身不限制并发。
- 优先用 join_next 循环收结果，别无脑用 join_all（一 panic 全炸）。
- 任务必须 Send + 'static，跨作用域用 Arc 克隆。
- 不要手动 await JoinSet 里面的 JoinHandle，会双重等待 panic。
