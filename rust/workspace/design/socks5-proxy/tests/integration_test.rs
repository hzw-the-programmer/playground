use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

async fn start_echo_server(port: u16) -> (JoinHandle<()>, SocketAddr) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                loop {
                    let n = stream.read(&mut buf).await.unwrap();
                    if n == 0 {
                        break;
                    }
                    stream.write_all(&buf[..n]).await.unwrap();
                }
            });
        }
    });

    (handle, addr)
}

async fn start_proxy(enable_auth: bool) -> (JoinHandle<()>, SocketAddr) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        loop {
            let (stream, peer) = listener.accept().await.unwrap();
            let enable_auth = enable_auth;
            tokio::spawn(async move {
                if let Err(e) =
                    socks5_proxy::server::handle_client(stream, enable_auth, "testuser", "testpass")
                        .await
                {
                    eprintln!("Error handling {}: {}", peer, e);
                }
            });
        }
    });

    (handle, addr)
}

async fn socks5_client(
    proxy_addr: SocketAddr,
    target_host: &str,
    target_port: u16,
    message: &[u8],
    auth: Option<(&str, &str)>,
) -> Result<Vec<u8>, String> {
    let mut stream = TcpStream::connect(proxy_addr)
        .await
        .map_err(|e| e.to_string())?;

    let mut methods = vec![0x00];
    if auth.is_some() {
        methods.push(0x02);
    }

    let mut handshake = vec![0x05, methods.len() as u8];
    handshake.extend(methods);
    stream
        .write_all(&handshake)
        .await
        .map_err(|e| e.to_string())?;

    let mut resp = [0u8; 2];
    stream
        .read_exact(&mut resp)
        .await
        .map_err(|e| e.to_string())?;
    if resp[0] != 0x05 {
        return Err("Invalid SOCKS5 version".into());
    }
    let method = resp[1];
    if method == 0x02 {
        let (user, pass) = auth.unwrap();
        let mut auth_req = vec![0x01, user.len() as u8];
        auth_req.extend_from_slice(user.as_bytes());
        auth_req.push(pass.len() as u8);
        auth_req.extend_from_slice(pass.as_bytes());
        stream
            .write_all(&auth_req)
            .await
            .map_err(|e| e.to_string())?;

        let mut auth_resp = [0u8; 2];
        stream
            .read_exact(&mut auth_resp)
            .await
            .map_err(|e| e.to_string())?;
        if auth_resp[0] != 0x01 || auth_resp[1] != 0x00 {
            return Err("Authentication failed".into());
        }
    } else if method != 0x00 {
        return Err("Unsupported method".into());
    }

    let mut request = vec![0x05, 0x01, 0x00];
    request.push(0x03);
    request.push(target_host.as_bytes().len() as u8);
    request.extend_from_slice(target_host.as_bytes());
    request.extend_from_slice(&target_port.to_be_bytes());
    stream
        .write_all(&request)
        .await
        .map_err(|e| e.to_string())?;

    let mut resp_buf = [0u8; 10];
    stream
        .read_exact(&mut resp_buf)
        .await
        .map_err(|e| e.to_string())?;
    if resp_buf[0] != 0x05 || resp_buf[1] != 0x00 {
        return Err(format!("Request failed, rep={}", resp_buf[1]));
    }

    stream.write_all(message).await.map_err(|e| e.to_string())?;
    stream.flush().await.map_err(|e| e.to_string())?;

    let mut recv_buf = vec![0u8; message.len()];
    stream
        .read_exact(&mut recv_buf)
        .await
        .map_err(|e| e.to_string())?;
    Ok(recv_buf)
}

#[tokio::test]
async fn test_no_auth_forward() {
    let (echo_handle, echo_addr) = start_echo_server(0).await;
    let (proxy_handle, proxy_addr) = start_proxy(false).await;

    let msg = b"HELLO SOCKS5";
    let recv = socks5_client(proxy_addr, "127.0.0.1", echo_addr.port(), msg, None)
        .await
        .unwrap();
    assert_eq!(recv, msg);

    proxy_handle.abort();
    echo_handle.abort();
}

#[tokio::test]
async fn test_with_auth_success() {
    let (echo_handle, echo_addr) = start_echo_server(0).await;
    let (proxy_handle, proxy_addr) = start_proxy(true).await;

    let msg = b"Authenticated request";
    let recv = socks5_client(
        proxy_addr,
        "127.0.0.1",
        echo_addr.port(),
        msg,
        Some(("testuser", "testpass")),
    )
    .await
    .unwrap();
    assert_eq!(recv, msg);

    proxy_handle.abort();
    echo_handle.abort();
}

#[tokio::test]
async fn test_with_auth_failure() {
    let (proxy_handle, proxy_addr) = start_proxy(true).await;

    let resp = socks5_client(
        proxy_addr,
        "127.0.0.1",
        9999,
        b"test",
        Some(("wrong", "wrong")),
    )
    .await;
    assert!(resp.is_err());

    proxy_handle.abort();
}

#[tokio::test]
async fn test_unsupported_command() {
    let (proxy_handle, proxy_addr) = start_proxy(false).await;

    let mut stream = TcpStream::connect(proxy_addr).await.unwrap();
    stream.write_all(&[0x05, 0x01, 0x00]).await.unwrap();

    let mut resp = [0u8; 2];
    stream.read_exact(&mut resp).await.unwrap();
    assert_eq!(resp, [0x05, 0x00]);

    // let request = [0x05, 0x02, 0x00, 0x01, 127, 0, 0, 1, 0x00, 0x50];
    let request = [0x05, 0x02, 0x00, 0x01];
    stream.write_all(&request).await.unwrap();

    let mut resp_buf = [0u8; 10];
    stream.read_exact(&mut resp_buf).await.unwrap();
    assert_eq!(resp_buf[1], 0x07);

    proxy_handle.abort();
}

#[tokio::test]
async fn test_connection_refused() {
    let (proxy_handle, proxy_addr) = start_proxy(false).await;

    let res = socks5_client(proxy_addr, "127.0.0.1", 12345, b"test", None).await;
    assert!(res.is_err());

    proxy_handle.abort();
}

#[tokio::test]
async fn test_unsupported_atyp() {
    let (proxy_handle, proxy_addr) = start_proxy(false).await;

    let mut stream = TcpStream::connect(proxy_addr).await.unwrap();
    stream.write_all(&[0x05, 0x01, 0x00]).await.unwrap();

    let mut resp = [0u8; 2];
    stream.read_exact(&mut resp).await.unwrap();
    assert_eq!(resp, [0x05, 0x00]);

    // let request = [0x05, 0x01, 0x00, 0x05, 127, 0, 0, 1, 0x00, 0x50];
    let request = [0x05, 0x01, 0x00, 0x05];
    stream.write_all(&request).await.unwrap();

    let mut resp_buf = [0u8; 10];
    stream.read_exact(&mut resp_buf).await.unwrap();
    assert_eq!(resp_buf[1], 0x08);

    proxy_handle.abort();
}
