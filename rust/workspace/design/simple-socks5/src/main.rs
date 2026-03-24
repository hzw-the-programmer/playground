use tokio::net::TcpListener;

use simple_socks5::{Result, handle_client};

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
