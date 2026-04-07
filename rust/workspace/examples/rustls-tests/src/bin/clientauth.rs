use rustls::{
    ClientConfig, RootCertStore,
    pki_types::{CertificateDer, PrivateKeyDer, ServerName},
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::TlsConnector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connector = TlsConnector::from(build_client_config_with_auth()?);

    let domain = "www.rust-lang.org";
    let tcp_stream = TcpStream::connect(format!("{}:443", domain)).await?;
    let server_name = ServerName::try_from(domain)?.to_owned();
    let mut tls_stream = connector.connect(server_name, tcp_stream).await?;

    let request = format!(
        "GET / HTTP/1.1\r\n\
        Host: {}\r\n\
        Connections: close\r\n\
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

fn build_client_config_with_auth() -> anyhow::Result<Arc<ClientConfig>> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let cert_chain = load_certs(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self-signed-certs")
            .join("cert.crt"),
    )?;
    let private_key = load_private_key(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self-signed-certs")
            .join("key.pem"),
    )?;

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_client_auth_cert(cert_chain, private_key)?;

    Ok(Arc::new(config))
}

fn load_certs(path: impl AsRef<Path>) -> anyhow::Result<Vec<CertificateDer<'static>>> {
    let cert_data = fs::read(path)?;
    let certs = rustls_pemfile::certs(&mut cert_data.as_slice()).collect::<Result<Vec<_>, _>>()?;
    Ok(certs)
}

fn load_private_key(path: impl AsRef<Path>) -> anyhow::Result<PrivateKeyDer<'static>> {
    let key_data = fs::read(path)?;
    let key = rustls_pemfile::private_key(&mut key_data.as_slice())?.unwrap();
    Ok(key)
}
