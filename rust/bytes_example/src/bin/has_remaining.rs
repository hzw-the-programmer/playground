use bytes::Buf;

fn main() {
    let s = "h";
    let mut buf = s.as_bytes(); 
    assert!(buf.has_remaining());
    buf.get_u8();
    assert!(!buf.has_remaining());
    assert_eq!("h", s);
}
