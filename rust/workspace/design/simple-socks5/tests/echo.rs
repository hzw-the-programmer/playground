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
                let mut buf = [0; 1024];
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

async fn start_proxy() -> (JoinHandle<()>, SocketAddr) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                simple_socks5::handle_client(stream).await;
            });
        }
    });

    (handle, addr)
}

async fn socks5_client(
    proxy_addr: SocketAddr,
    target_host: &str,
    target_port: u16,
    msg: &[u8],
) -> Result<Vec<u8>, String> {
    let mut stream = TcpStream::connect(proxy_addr)
        .await
        .map_err(|e| e.to_string())?;

    let target_host_bytes = target_host.as_bytes();
    let mut request = Vec::with_capacity(1 + target_host_bytes.len() + 2);
    request.push(target_host_bytes.len() as u8);
    request.extend_from_slice(target_host_bytes);
    request.extend_from_slice(&target_port.to_be_bytes());
    stream
        .write_all(&request)
        .await
        .map_err(|e| e.to_string())?;

    stream.write_all(msg).await.map_err(|e| e.to_string())?;
    let mut recv_buf = vec![0; msg.len()];
    stream
        .read_exact(&mut recv_buf)
        .await
        .map_err(|e| e.to_string())?;
    Ok(recv_buf)
}

#[tokio::test]
async fn test_echo() {
    let (echo_handle, echo_addr) = start_echo_server(0).await;
    let (proxy_handle, proxy_addr) = start_proxy().await;

    let msg = b"HELLO SOCKS5";
    let recv = socks5_client(proxy_addr, "127.0.0.1", echo_addr.port(), msg)
        .await
        .unwrap();
    assert_eq!(recv, msg);

    proxy_handle.abort();
    echo_handle.abort();
}
