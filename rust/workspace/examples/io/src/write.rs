pub fn test() {
    // test1();
    test2();
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
