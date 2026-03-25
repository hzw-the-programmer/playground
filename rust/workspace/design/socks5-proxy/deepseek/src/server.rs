// src/server.rs
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::error::Socks5Error;
use crate::auth::authenticate;

// SOCKS5 常量
const SOCKS5_VERSION: u8 = 0x05;
const SOCKS5_CMD_CONNECT: u8 = 0x01;
const SOCKS5_ATYP_IPV4: u8 = 0x01;
const SOCKS5_ATYP_DOMAIN: u8 = 0x03;
const SOCKS5_ATYP_IPV6: u8 = 0x04;

// 响应码
const SOCKS5_REP_SUCCESS: u8 = 0x00;
const SOCKS5_REP_GENERAL_FAILURE: u8 = 0x01;
const SOCKS5_REP_CONNECTION_NOT_ALLOWED: u8 = 0x02;
const SOCKS5_REP_NETWORK_UNREACHABLE: u8 = 0x03;
const SOCKS5_REP_HOST_UNREACHABLE: u8 = 0x04;
const SOCKS5_REP_CONNECTION_REFUSED: u8 = 0x05;
const SOCKS5_REP_TTL_EXPIRED: u8 = 0x06;
const SOCKS5_REP_COMMAND_NOT_SUPPORTED: u8 = 0x07;
const SOCKS5_REP_ADDRESS_TYPE_NOT_SUPPORTED: u8 = 0x08;

/// 处理客户端连接的主函数
pub async fn handle_client(mut client: TcpStream, enable_auth: bool, username: &str, password: &str) -> Result<(), Socks5Error> {
    // 1. 握手，选择认证方法
    let method = perform_handshake(&mut client, enable_auth).await?;
    if method == 0x02 {
        // 用户名密码认证
        if !authenticate(&mut client, username, password).await? {
            return Err(Socks5Error::AuthFailed);
        }
    }

    // 2. 解析请求
    let (host, port) = parse_request(&mut client).await?;

    // 3. 连接到目标
    let target = connect_to_target(&host, port).await?;

    // 4. 发送成功响应
    send_response(&mut client, SOCKS5_REP_SUCCESS).await?;

    // 5. 双向转发
    let (client_reader, client_writer) = client.into_split();
    let (target_reader, target_writer) = target.into_split();

    let forward_client = tokio::spawn(forward_data(client_reader, target_writer));
    let forward_target = tokio::spawn(forward_data(target_reader, client_writer));

    tokio::select! {
        _ = forward_client => {},
        _ = forward_target => {},
    }

    Ok(())
}

/// SOCKS5 握手，返回选中的方法
async fn perform_handshake(client: &mut TcpStream, enable_auth: bool) -> Result<u8, Socks5Error> {
    let mut buf = [0u8; 2];
    client.read_exact(&mut buf).await?;
    if buf[0] != SOCKS5_VERSION {
        return Err(Socks5Error::Protocol("Invalid SOCKS version".into()));
    }
    let nmethods = buf[1];
    if nmethods == 0 {
        return Err(Socks5Error::Protocol("No authentication methods offered".into()));
    }
    let mut methods = vec![0u8; nmethods as usize];
    client.read_exact(&mut methods).await?;

    // 确定使用的方法：优先无认证，如果启用认证且客户端支持则使用认证
    let selected = if enable_auth && methods.contains(&0x02) {
        0x02
    } else if methods.contains(&0x00) {
        0x00
    } else {
        // 不支持任何方法
        client.write_all(&[SOCKS5_VERSION, 0xFF]).await?;
        return Err(Socks5Error::Protocol("No acceptable authentication method".into()));
    };
    client.write_all(&[SOCKS5_VERSION, selected]).await?;
    Ok(selected)
}

/// 解析 SOCKS5 请求，返回目标地址和端口
async fn parse_request(client: &mut TcpStream) -> Result<(String, u16), Socks5Error> {
    let mut header = [0u8; 4];
    client.read_exact(&mut header).await?;
    let ver = header[0];
    let cmd = header[1];
    let atyp = header[3];

    if ver != SOCKS5_VERSION {
        return Err(Socks5Error::Protocol("Invalid SOCKS version in request".into()));
    }
    if cmd != SOCKS5_CMD_CONNECT {
        // 对于不支持的命令，返回错误码
        let code = match cmd {
            0x02 => SOCKS5_REP_COMMAND_NOT_SUPPORTED, // BIND
            0x03 => SOCKS5_REP_COMMAND_NOT_SUPPORTED, // UDP ASSOCIATE
            _ => SOCKS5_REP_COMMAND_NOT_SUPPORTED,
        };
        send_response(client, code).await?;
        return Err(Socks5Error::UnsupportedCommand(cmd));
    }

    let host = match atyp {
        SOCKS5_ATYP_IPV4 => {
            let mut addr = [0u8; 4];
            client.read_exact(&mut addr).await?;
            Ipv4Addr::from(addr).to_string()
        }
        SOCKS5_ATYP_IPV6 => {
            let mut addr = [0u8; 16];
            client.read_exact(&mut addr).await?;
            Ipv6Addr::from(addr).to_string()
        }
        SOCKS5_ATYP_DOMAIN => {
            let mut len_buf = [0u8; 1];
            client.read_exact(&mut len_buf).await?;
            let domain_len = len_buf[0] as usize;
            let mut domain = vec![0u8; domain_len];
            client.read_exact(&mut domain).await?;
            String::from_utf8(domain).map_err(|_| Socks5Error::Protocol("Invalid domain name".into()))?
        }
        _ => {
            send_response(client, SOCKS5_REP_ADDRESS_TYPE_NOT_SUPPORTED).await?;
            return Err(Socks5Error::UnsupportedAddressType(atyp));
        }
    };

    let mut port_buf = [0u8; 2];
    client.read_exact(&mut port_buf).await?;
    let port = u16::from_be_bytes(port_buf);

    Ok((host, port))
}

/// 连接到目标服务器
async fn connect_to_target(host: &str, port: u16) -> Result<TcpStream, Socks5Error> {
    let addr = format!("{}:{}", host, port);
    match tokio::net::TcpStream::connect(&addr).await {
        Ok(stream) => Ok(stream),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::ConnectionRefused {
                Err(Socks5Error::TargetRefused)
            } else if e.kind() == std::io::ErrorKind::TimedOut {
                Err(Socks5Error::TargetConnectionFailed)
            } else {
                Err(Socks5Error::NetworkUnreachable)
            }
        }
    }
}

/// 发送 SOCKS5 响应（绑定地址填 0）
async fn send_response(client: &mut TcpStream, rep: u8) -> Result<(), Socks5Error> {
    let response = [
        SOCKS5_VERSION, rep, 0x00,
        SOCKS5_ATYP_IPV4, // 使用 IPv4 填充
        0, 0, 0, 0,      // BND.ADDR = 0.0.0.0
        0, 0,            // BND.PORT = 0
    ];
    client.write_all(&response).await?;
    Ok(())
}

/// 双向数据转发
async fn forward_data(mut reader: impl tokio::io::AsyncRead + Unpin, mut writer: impl tokio::io::AsyncWrite + Unpin) {
    let mut buf = [0u8; 8192];
    loop {
        let n = match reader.read(&mut buf).await {
            Ok(n) => n,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }
        if writer.write_all(&buf[..n]).await.is_err() {
            break;
        }
        if writer.flush().await.is_err() {
            break;
        }
    }
}