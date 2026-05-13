use std::time::Duration;
use tokio::task::JoinSet;

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
