// src/error.rs
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Socks5Error {
    #[error("IO error: {0}")]
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

impl Socks5Error {
    /// 转换为 SOCKS5 响应码
    pub fn to_reply_code(&self) -> u8 {
        match self {
            Socks5Error::AuthFailed => 0x01, // 通用失败，实际认证失败应在认证阶段返回 0x01
            Socks5Error::UnsupportedCommand(_) => 0x07,
            Socks5Error::UnsupportedAddressType(_) => 0x08,
            Socks5Error::TargetConnectionFailed => 0x01, // 通用失败
            Socks5Error::TargetRefused => 0x05,
            Socks5Error::NetworkUnreachable => 0x03,
            _ => 0x01, // 其他错误通用失败
        }
    }
}