use std::io::Read;

pub fn test() {
    test1();
}

fn test1() {
    let a = [1, 2, 3];
    let mut s = &a[..];
    let mut b = [0; 2];
    let n = s.read(&mut b).unwrap();
    assert_eq!(n, 2);
    assert_eq!(b, [1, 2]);
    assert_eq!(s, [3]);
    let n = s.read(&mut b).unwrap();
    assert_eq!(n, 1);
    assert_eq!(b, [3, 2]);
    assert_eq!(s, []);
}
