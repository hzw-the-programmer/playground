use rustls::{ClientConfig, RootCertStore, pki_types::ServerName};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::TlsConnector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    let domain = "www.rust-lang.org";
    let tcp_stream = TcpStream::connect(format!("{}:443", domain)).await?;
    let server_name = ServerName::try_from(domain)?.to_owned();
    let mut tls_stream = connector.connect(server_name, tcp_stream).await?;

    let request = format!(
        "GET / HTTP/1.1\r\n\
        Host: {}\r\n\
        Connection: close\r\n\
        \r\n",
        domain
    );

    tls_stream.write_all(request.as_bytes()).await?;
    tls_stream.flush().await?;

    let mut response = String::new();
    tls_stream.read_to_string(&mut response).await?;

    println!("{}", response);

    Ok(())
}
