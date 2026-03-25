use std::io::ErrorKind;
use std::marker::Unpin;
use std::net::{Ipv4Addr, Ipv6Addr};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::auth::authenticate;
use crate::error::Socks5Error;

const SOCKS5_VERSION: u8 = 0x05;
const SOCKS5_CMD_CONNECT: u8 = 0x01;
const SOCKS5_ATYP_IPV4: u8 = 0x01;
const SOCKS5_ATYP_DOMAIN: u8 = 0x03;
const SOCKS5_ATYP_IPV6: u8 = 0x04;

const SOCKS5_REP_SUCCESS: u8 = 0x00;
const SOCKS5_REP_COMMAND_NOT_SUPPORTED: u8 = 0x07;
const SOCKS5_REP_ADDRESS_TYPE_NOT_SUPPORTED: u8 = 0x08;

pub async fn handle_client(
    mut client: TcpStream,
    enable_auth: bool,
    username: &str,
    password: &str,
) -> Result<(), Socks5Error> {
    let method = perform_handshake(&mut client, enable_auth).await?;
    if method == 0x02 {
        if !authenticate(&mut client, username, password).await? {
            return Err(Socks5Error::AuthFailed);
        }
    }

    let (host, port) = parse_request(&mut client).await?;

    let target = connect_to_target(&host, port).await?;

    send_response(&mut client, SOCKS5_REP_SUCCESS).await?;

    let (client_reader, client_writer) = client.into_split();
    let (target_reader, target_writer) = target.into_split();

    let forward_client = tokio::spawn(forward_data(client_reader, target_writer));
    let forward_target = tokio::spawn(forward_data(target_reader, client_writer));

    tokio::select! {
        _ = forward_client => {}
        _ = forward_target => {}
    }

    Ok(())
}

async fn perform_handshake(client: &mut TcpStream, enable_auth: bool) -> Result<u8, Socks5Error> {
    let mut buf = [0u8; 2];

    client.read_exact(&mut buf).await?;

    if buf[0] != SOCKS5_VERSION {
        return Err(Socks5Error::Protocol("Invalid SOCKS version".into()));
    }

    let nmethods = buf[1];
    if nmethods == 0 {
        return Err(Socks5Error::Protocol(
            "No authentication methods offered".into(),
        ));
    }

    let mut methods = vec![0u8; nmethods as usize];
    client.read_exact(&mut methods).await?;

    let selected = if enable_auth && methods.contains(&0x02) {
        0x02
    } else if methods.contains(&0x00) {
        0x00
    } else {
        client.write_all(&[SOCKS5_VERSION, 0xFF]).await?;
        return Err(Socks5Error::Protocol(
            "No acceptable authentication method".into(),
        ));
    };
    client.write_all(&[SOCKS5_VERSION, selected]).await?;
    Ok(selected)
}

async fn parse_request(client: &mut TcpStream) -> Result<(String, u16), Socks5Error> {
    let mut header = [0u8; 4];
    client.read_exact(&mut header).await?;
    let ver = header[0];
    let cmd = header[1];
    let atyp = header[3];

    if ver != SOCKS5_VERSION {
        return Err(Socks5Error::Protocol(
            "Invalid SOCKS5 version in request".into(),
        ));
    }

    if cmd != SOCKS5_CMD_CONNECT {
        let code = match cmd {
            0x02 => SOCKS5_REP_COMMAND_NOT_SUPPORTED,
            0x03 => SOCKS5_REP_COMMAND_NOT_SUPPORTED,
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
            String::from_utf8(domain)
                .map_err(|_| Socks5Error::Protocol("Invalid domain name".into()))?
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

async fn connect_to_target(host: &str, port: u16) -> Result<TcpStream, Socks5Error> {
    let addr = format!("{}:{}", host, port);
    TcpStream::connect(&addr).await.map_err(|e| match e.kind() {
        ErrorKind::ConnectionRefused => Socks5Error::TargetRefused,
        ErrorKind::TimedOut => Socks5Error::TargetConnectionFailed,
        _ => Socks5Error::NetworkUnreachable,
    })
}

async fn send_response(client: &mut TcpStream, rep: u8) -> Result<(), Socks5Error> {
    #[rustfmt::skip]
    let response = [
        SOCKS5_VERSION, rep, 0x00,
        SOCKS5_ATYP_IPV4,
        0, 0, 0, 0,
        0, 0,
    ];

    client.write_all(&response).await?;
    Ok(())
}

async fn forward_data(mut reader: impl AsyncRead + Unpin, mut writer: impl AsyncWrite + Unpin) {
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
