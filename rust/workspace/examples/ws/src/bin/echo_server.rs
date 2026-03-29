use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("WebSocket 服务端：ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("握手失败：{e}");
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    println!("新连接");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) if msg.is_text() || msg.is_binary() => {
                if write.send(msg).await.is_err() {
                    break;
                }
            }
            Ok(msg) if msg.is_close() => {
                println!("关闭连接");
                break;
            }
            Err(e) => {
                eprintln!("读取错误：{e}");
                break;
            }
            _ => {}
        }
    }
}
