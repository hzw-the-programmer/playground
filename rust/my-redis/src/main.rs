use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let r = client.get(&key).await;
                    let _ = resp.send(r);
                }
                Set { key, val, resp } => {
                    let r = client.set(&key, val).await;
                    let _ = resp.send(r);
                }
            }
        }
    });

    let task1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        tx.send(Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let resp = resp_rx.await.unwrap();
        println!("Got: {:?}", resp);
    });

    let task2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        tx2.send(Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let resp = resp_rx.await.unwrap();
        println!("Got: {:?}", resp);
    });

    manager.await.unwrap();
    task1.await.unwrap();
    task2.await.unwrap();
}

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
