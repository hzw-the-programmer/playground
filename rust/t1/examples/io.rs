use std::io::{self, BufRead};

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("\ntest1\n");
    let cursor = io::Cursor::new(b"lorem\nnipsum\r\ndolor");
    let mut iter = cursor.lines();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}

fn test2() {
    println!("\ntest2\n");
    let cursor = io::Cursor::new(b"lorem\nnipsum\r\ndolor");
    let mut iter = cursor.lines();
    for i in iter {
        println!("{:?}", i);
    }
}
