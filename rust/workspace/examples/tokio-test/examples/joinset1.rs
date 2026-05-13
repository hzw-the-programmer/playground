use std::time::Duration;
use tokio::task::JoinSet;

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
