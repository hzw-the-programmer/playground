use std::{convert::Infallible, net::SocketAddr};

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, server::conn::http1, service::service_fn};
use hyper_util::rt::{TokioIo, TokioTimer};
use tokio::net::TcpListener;

async fn hello(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: http://{addr}");

    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("error serving connection: {err:?}");
            }
        });
    }
}
