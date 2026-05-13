use std::time::Duration;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        1
    });
    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        2
    });

    // 消费self，等待全部完成，返回完成顺序列表
    let all = set.join_all().await;
    println!("所有结果: {:?}", all);
}
