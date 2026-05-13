use std::time::Duration;
use tokio::select;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    set.spawn(async {
        tokio::time::sleep(Duration::from_secs(10)).await;
    });

    select! {
        _ = tokio::time::sleep(Duration::from_secs(3)) => {
            println!("超时，取消所有任务");
            set.abort_all();
        }
        _ = set.join_next() => {}
    }
}
