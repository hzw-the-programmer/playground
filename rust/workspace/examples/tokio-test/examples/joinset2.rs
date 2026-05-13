use std::time::Duration;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    // 初始任务
    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("初始任务完成");
    });
    println!("{}", set.len());

    // 动态添加：每完成一个就加一个新的
    while let Some(res) = set.join_next().await {
        res.unwrap();
        println!("{}", set.len());
        // if set.len() < 5 {
        set.spawn(async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("动态任务完成");
        });
        // }
        println!("{}", set.len());
    }
}
