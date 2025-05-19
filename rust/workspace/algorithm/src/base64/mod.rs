const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

mod decode;
pub use decode::decode;
mod encode;
pub use encode::encode;

#[cfg(test)]
mod tests;
