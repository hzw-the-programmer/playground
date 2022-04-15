use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

const ECHO_SERVER_ADDRESS: &str = "localhost:8080";

#[tokio::main]
async fn main() {
    println!("connecting to echo server {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        println!("conneted to echo server");
        println!("local address {}", stream.local_addr().unwrap());

        let msg = "Hello World";
        stream.write(msg.as_bytes()).await;
        stream.flush().await;
        println!("sent: {}", msg);

        let mut buf = [0;1024];
        stream.read(&mut buf).await;
        let msg = String::from_utf8_lossy(&buf);
        println!("received: {}", msg);
    } else {
        println!("failed to connect to echo server");
    }
}
