use bytes::Buf;

fn main() {
    let mut buf = &b"hello world"[..];
    assert_eq!(b'h', buf.get_u8());
    assert_eq!(b'e', buf.get_u8());
    assert_eq!(b'l', buf.get_u8());

    let mut rest = [0; 8];
    buf.copy_to_slice(&mut rest);
    assert_eq!(&b"lo world"[..], &rest[..]);
    assert_eq!(&b"lo world"[..], &rest);
    assert_eq!(&b"lo world"[..], rest);

    assert_eq!(0, buf.remaining());
}
