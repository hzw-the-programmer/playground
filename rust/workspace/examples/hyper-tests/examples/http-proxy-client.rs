use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy_url = std::env::args().nth(1).unwrap();
    let url = std::env::args().nth(2).unwrap();

    let proxy_url = proxy_url.parse::<hyper::Uri>().unwrap();
    let proxy_host = proxy_url.host().unwrap();
    let proxy_port = proxy_url.port_u16().unwrap();

    let stream = TcpStream::connect((proxy_host, proxy_port)).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::Builder::new()
        .handshake(io)
        .await?;

    tokio::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection failed: {err:?}");
        }
    });

    let url = url.parse::<hyper::Uri>().unwrap();
    let req = Request::builder().uri(url).body(Empty::<Bytes>::new())?;

    let mut res = sender.send_request(req).await?;

    println!("Response status: {}", res.status());
    println!("Headers: {:#?}", res.headers());

    while let Some(frame) = res.frame().await {
        let frame = frame?;
        if let Some(chunk) = frame.data_ref() {
            tokio::io::stdout().write_all(&chunk).await?;
        }
    }

    Ok(())
}
