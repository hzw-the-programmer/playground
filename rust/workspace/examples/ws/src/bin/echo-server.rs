use futures_util::{StreamExt, TryStreamExt, future};
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let _ = env_logger::try_init();
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("Listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer addr");
    info!("Peer address: {}", addr);

    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages");
}
