// src/main.rs
mod error;
mod auth;
mod server;

use tokio::net::TcpListener;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:1080";
    let listener = TcpListener::bind(addr).await?;
    println!("SOCKS5 proxy listening on {}", addr);

    // 配置：是否启用认证，用户名和密码
    let enable_auth = false; // 可改为 true
    let username = "user";
    let password = "pass";

    loop {
        let (stream, peer) = listener.accept().await?;
        println!("New connection from {}", peer);
        let enable_auth = enable_auth;
        let username = username.to_string();
        let password = password.to_string();
        tokio::spawn(async move {
            if let Err(e) = server::handle_client(stream, enable_auth, &username, &password).await {
                eprintln!("Error handling {}: {}", peer, e);
            }
        });
    }
}