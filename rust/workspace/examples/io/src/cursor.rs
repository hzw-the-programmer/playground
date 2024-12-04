use std::io::{Cursor, Read, Write};

pub fn test() {
    test1();
}

fn test1() {
    let mut c = Cursor::new(vec![1, 2, 3, 4, 5]);
    let mut b = [0; 2];
    assert_eq!(c.position(), 0);
    let _ = c.read(&mut b[..]);
    assert_eq!([1, 2], b);
    assert_eq!(c.position(), 2);
    let _ = c.write(&b[..]);
    assert_eq!(c.position(), 4);
    assert_eq!(c.get_ref(), &vec![1, 2, 1, 2, 5]);
}
