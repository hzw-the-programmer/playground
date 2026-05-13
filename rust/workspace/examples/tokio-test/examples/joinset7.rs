use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    let sem = Arc::new(Semaphore::new(3)); // 最大并发3

    for i in 1..=10 {
        let sem = sem.clone();
        set.spawn(async move {
            // 获取令牌，自动限流
            let _permit = sem.acquire().await.unwrap();

            // tokio::time::sleep(Duration::from_millis(200)).await;
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("处理完成: {}", i);
            i
        });
    }

    // 等待全部完成，忽略错误可 unwrap，严谨用match
    while let Some(res) = set.join_next().await {
        let _ = res;
    }
}
