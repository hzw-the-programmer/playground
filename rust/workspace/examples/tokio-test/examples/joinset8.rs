use std::time::Duration;
use tokio::task::JoinSet;

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
