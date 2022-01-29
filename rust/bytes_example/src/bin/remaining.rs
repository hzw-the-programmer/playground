use bytes::Buf;

fn main() {
    let mut buf = &b"hello world"[..];
    assert_eq!(11, buf.remaining());
    buf.get_u8();
    assert_eq!(10, buf.remaining());
}
