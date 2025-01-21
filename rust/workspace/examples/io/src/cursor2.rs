use std::io::Cursor;

pub fn test() {
    test1();
}

fn test1() {
    let mut buff = Cursor::new(vec![1, 2, 3, 4, 5]);

    assert_eq!(buff.position(), 0);
    assert_eq!(buff.remaining_slice(), &[1, 2, 3, 4, 5]);

    buff.set_position(2);
    assert_eq!(buff.remaining_slice(), &[3, 4, 5]);

    buff.set_position(4);
    assert_eq!(buff.remaining_slice(), &[5]);

    buff.set_position(6);
    assert_eq!(buff.remaining_slice(), &[]);
    // expected `i32`, found `&[_; 0]`
    // let i: i32 = &[];
}
