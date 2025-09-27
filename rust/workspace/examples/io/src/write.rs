pub fn test() {
    // test1();
    // test2();
    // vec_write();
    cursor_vec_write();
}

fn test1() {
    use std::io::Write;

    let mut a = [0; 16];
    let mut s = &mut a[..];
    let _ = s.write(b"hello");
    println!("{:?}", s);
    // println!("{:?}", a);
    let _ = s.write(b" world!");
    println!("{:?}", s);
    println!("{:?}", a);
}

fn test2() {
    use std::io::{Cursor, Write};

    let mut a = [0; 16];
    // let s = &mut a[..];
    // let mut c = Cursor::new(s);
    let mut c = Cursor::new(&mut a[..]);
    let _ = c.write(b"hello");
    // println!("{:?}", s);
    println!("{:?}", a);
}

fn vec_write() {
    use std::io::Write;

    let mut v = vec![1, 2, 3];
    let _ = v.write(&[4, 5, 6][..]);
    println!("{:?}", v);
}

fn cursor_vec_write() {
    use std::io::{Cursor, Write};

    let mut c = Cursor::new(vec![1, 2, 3]);
    let _ = c.write(&[4, 5, 6][..]);
    println!("{:?}", c.into_inner());
}
