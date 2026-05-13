use std::time::Duration;
use tokio::task::JoinSet;

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
