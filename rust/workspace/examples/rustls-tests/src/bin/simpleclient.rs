use rustls::{ClientConfig, ClientConnection, RootCertStore, Stream, pki_types::ServerName};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

fn main() -> anyhow::Result<()> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = Arc::new(
        ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth(),
    );

    let domain = "www.rust-lang.org";
    let addr = format!("{}:443", domain);
    let mut tcp_stream = TcpStream::connect(addr)?;

    let server_name = ServerName::try_from(domain)?.to_owned();
    let mut client_conn = ClientConnection::new(config, server_name)?;
    let mut tls_stream = Stream::new(&mut client_conn, &mut tcp_stream);

    let request = format!(
        "GET / HTTP/1.1\r\n\
        Host: {}\r\n\
        Connection: close\r\n\
        \r\n",
        domain
    );

    tls_stream.write_all(request.as_bytes())?;
    tls_stream.flush()?;

    let ciphersuite = tls_stream.conn.negotiated_cipher_suite().unwrap();
    writeln!(
        &mut std::io::stderr(),
        "Current ciphersuite: {:?}",
        ciphersuite.suite()
    )?;

    let mut plaintext = Vec::new();
    tls_stream.read_to_end(&mut plaintext)?;

    std::io::stdout().write_all(&plaintext)?;

    Ok(())
}
