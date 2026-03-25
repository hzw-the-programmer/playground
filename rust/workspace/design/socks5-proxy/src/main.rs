use anyhow::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:1080";
    let listener = TcpListener::bind(addr).await?;
    println!("SOCKS5 proxy listening on {}", addr);

    let enable_auth = false;
    let username = "user";
    let password = "pass";

    loop {
        let (stream, peer) = listener.accept().await?;
        println!("New connection from {}", peer);

        let enable_auth = enable_auth;
        let username = username.to_string();
        let password = password.to_string();

        tokio::spawn(async move {
            if let Err(e) =
                socks5_proxy::server::handle_client(stream, enable_auth, &username, &password).await
            {
                eprintln!("Error handling {}: {}", peer, e);
            }
        });
    }
}
