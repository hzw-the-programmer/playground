use bytes::Buf;

fn main() {
    let mut buf = &b"hello world"[..];
    assert_eq!(buf.chunk(), b"hello world");
    buf.advance(6);
    assert_eq!(buf.chunk(), b"world");
}
