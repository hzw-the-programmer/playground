use std::io::{Cursor, Read, Write};

pub fn test() {
    // test1();
    test2();
}

fn test1() {
    let mut c = Cursor::new(vec![1, 2, 3, 4, 5]);
    let mut b = [0; 2];
    assert_eq!(c.position(), 0);
    let n = c.read(&mut b[..]).unwrap();
    assert_eq!(n, 2);
    assert_eq!(b, [1, 2]);
    assert_eq!(c.position(), 2);
    let n = c.write(&b[..]).unwrap();
    assert_eq!(n, 2);
    assert_eq!(c.position(), 4);
    assert_eq!(c.remaining_slice(), &[5]);
    assert_eq!(c.get_ref(), &vec![1, 2, 1, 2, 5]);
}

fn test2() {
    let mut c = Cursor::new(vec![1, 2, 3]);
    assert_eq!(c.position(), 0);
    c.set_position(4);
    let n = c.write(&[0; 1][0..0]).unwrap();
    assert_eq!(n, 0);
    assert_eq!(c.get_ref(), &[1, 2, 3, 0]);
    assert_eq!(c.position(), 4);
}
