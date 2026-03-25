use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::Socks5Error;

pub async fn authenticate(
    client: &mut TcpStream,
    username: &str,
    password: &str,
) -> Result<bool, Socks5Error> {
    let mut ver = [0u8; 1];
    client.read_exact(&mut ver).await?;
    if ver[0] != 0x01 {
        return Err(Socks5Error::Protocol("Invalid auth version".into()));
    }

    let mut ulen = [0u8; 1];
    client.read_exact(&mut ulen).await?;
    let mut uname = vec![0u8; ulen[0] as usize];
    client.read_exact(&mut uname).await?;

    let mut plen = [0u8; 1];
    client.read_exact(&mut plen).await?;
    let mut passwd = vec![0u8; plen[0] as usize];
    client.read_exact(&mut passwd).await?;

    let ok = uname == username.as_bytes() && passwd == password.as_bytes();
    if ok {
        client.write_all(&[0x01, 0x00]).await?;
        Ok(true)
    } else {
        client.write_all(&[0x01, 0x01]).await?;
        Err(Socks5Error::AuthFailed)
    }
}
