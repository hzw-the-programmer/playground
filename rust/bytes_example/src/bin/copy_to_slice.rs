use bytes::Buf;

//fn check(i: i32) {}

fn main() {
    let s = "hello world";
    let mut buf = s.as_bytes();
    let mut b = [0; 5];
    //check(&b);
    //check(&mut b);
    //check(b);
    buf.copy_to_slice(&mut b);
    assert_eq!(&b"hello"[..], b);
    assert_eq!(b, &b"hello"[..]);
    assert_eq!(6, buf.remaining());
}
