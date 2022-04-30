use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    for i in 0..2 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("handle {}", i)).await;
        });
    }

    drop(tx);

    while let Some(msg) = rx.recv().await {
        println!("received: {}", msg);
    }
}
