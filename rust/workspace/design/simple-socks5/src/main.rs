use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, copy_bidirectional};
use tokio::net::{TcpListener, TcpStream};

/*
设计一个异步 TCP 代理服务器（SOCKS5 简化版）

题目要求
实现一个异步 TCP 代理，支持：
1. 监听指定端口，接收客户端 TCP 连接；
2. 解析客户端的「目标地址 + 端口」请求（简化版 SOCKS5，无需认证）；
3. 建立到目标服务器的 TCP 连接；
4. 双向转发客户端 ↔ 目标服务器的数据流；
5. 高并发（支持数千个并发连接）；
6. 优雅处理连接关闭 / 错误。

设计思路
* 异步运行时：使用 tokio（成熟的异步 IO 运行时）；
* 核心抽象：基于 AsyncRead/AsyncWrite 实现通用数据转发；
* 并发模型：每个客户端连接生成一个异步任务（tokio::spawn）；
* 数据转发：使用 tokio::io::copy_bidirectional 实现高效双向转发；
* 错误处理：捕获并记录 IO 错误，优雅关闭连接。

关键考点
* 异步 IO（AsyncRead/AsyncWrite）；
* 高并发异步任务管理；
* 双向数据流转发；
* 错误处理与资源清理；
* TCP 网络编程最佳实践。
*/

// 自定义错误类型
#[derive(Error, Debug)]
enum ProxyError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Address parse error: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
}

type Result<T> = std::result::Result<T, ProxyError>;

#[tokio::main]
async fn main() -> Result<()> {
    // 监听 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Proxy server listening on 127.0.0.1:8080");

    // 接受客户端连接
    loop {
        let (stream, _) = listener.accept().await?;
        // 每个连接生成一个异步任务
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}

// 处理单个客户连接
async fn handle_client(mut client_stream: TcpStream) {
    let client_addr = match client_stream.peer_addr() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to get client addr: {}", e);
            return;
        }
    };

    println!("New client connected: {}", client_addr);

    // 解析客户端请求
    let target_addr = match parse_client_request(&mut client_stream).await {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to parse request from {}: {}", client_addr, e);
            let _ = client_stream.shutdown().await;
            return;
        }
    };

    println!(
        "Client {} wants to connect to: {}",
        client_addr, target_addr
    );

    // 连接目标服务器
    let target_stream = match TcpStream::connect(target_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", target_addr, e);
            let _ = client_stream.shutdown().await;
            return;
        }
    };

    // 双向转发数据
    if let Err(e) = forward_data(client_stream, target_stream).await {
        eprintln!("Forward error ({} ↔ {}): {}", client_addr, target_addr, e);
    }

    println!("Connection closed: {}", client_addr);
}

// 简化版请求格式：【目标地址长度（1）】+【目标地址】+【端口（2）】
async fn parse_client_request(stream: &mut TcpStream) -> Result<SocketAddr> {
    // 读取目标地址长度
    let mut addr_len_buf = [0; 1];
    stream.read_exact(&mut addr_len_buf).await?;
    let addr_len = addr_len_buf[0] as usize;

    // 读取目标地址
    let mut addr_buf = vec![0; addr_len];
    stream.read_exact(&mut addr_buf).await?;
    let addr = String::from_utf8_lossy(&addr_buf).to_string();

    // 读取端口（大端序）
    let mut port_buf = [0; 2];
    stream.read_exact(&mut port_buf).await?;
    let port = u16::from_be_bytes(port_buf);

    // 构造 SocketAddr
    let addr = format!("{}:{}", addr, port).parse()?;
    Ok(addr)
}

// 双向数据转发
async fn forward_data(mut client_stream: TcpStream, mut target_stream: TcpStream) -> Result<()> {
    // 使用 tokio 内置的双向拷贝（高效）
    let (client_to_target, target_to_client) =
        copy_bidirectional(&mut client_stream, &mut target_stream).await?;

    println!(
        "Forward: {} bytes (client → target), {} bytes (target → client)",
        client_to_target, target_to_client
    );

    // 优雅关闭连接
    let _ = client_stream.shutdown().await?;
    let _ = target_stream.shutdown().await?;

    Ok(())
}
