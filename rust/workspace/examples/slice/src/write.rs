use std::io::Write;

pub fn test() {
    test1();
}

fn test1() {
    let mut a = [0; 3];
    let mut s = &mut a[..];
    let b = [1, 2];
    let n = s.write(&b[..]).unwrap();
    assert_eq!(n, 2);
    assert_eq!(s, [0]);
    // assert_eq!(a, [1, 2, 0]);
    let n = s.write(&b[..]).unwrap();
    assert_eq!(n, 1);
    assert_eq!(s, []);
    assert_eq!(a, [1, 2, 1]);
}
