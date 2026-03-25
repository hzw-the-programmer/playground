use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Socks5Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Unsupported command: {0}")]
    UnsupportedCommand(u8),
    #[error("Unsupported address type: {0}")]
    UnsupportedAddressType(u8),
    #[error("Connection to target failed")]
    TargetConnectionFailed,
    #[error("Target connection refused")]
    TargetRefused,
    #[error("Network unreachable")]
    NetworkUnreachable,
}
