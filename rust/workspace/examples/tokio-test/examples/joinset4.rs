use tokio::task::JoinSet;

async fn batch_process<T, F, Fut>(
    items: Vec<T>,
    f: F,
) -> Vec<Result<Fut::Output, tokio::task::JoinError>>
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

#[tokio::main]
async fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let results = batch_process(items, |i| async move {
        println!("{i}");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        i
    })
    .await;
    println!("{:?}", results);
}
