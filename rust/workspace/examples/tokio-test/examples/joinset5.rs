use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

async fn limited_concurrent<T, F, Fut>(
    items: Vec<T>,
    max_concurrent: usize,
    f: F,
) -> Vec<Fut::Output>
where
    T: Send + 'static,
    F: Fn(T) -> Fut + Send + 'static + Clone,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    let sem = Arc::new(Semaphore::new(max_concurrent));
    let mut set = JoinSet::new();

    for item in items {
        let sem = sem.clone();
        let f = f.clone();
        set.spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            f(item).await
        });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res.unwrap());
    }
    results
}

#[tokio::main]
async fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let results = limited_concurrent(items, 2, |i| async move {
        println!("{i}");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        i
    })
    .await;
    println!("{:?}", results);
}
