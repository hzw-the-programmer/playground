use std::io::{self, BufRead};

fn main() {
    test1();
    test2();
    test3();
    test4();
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

fn test3() {
    println!("\ntest3\n");
    let cursor = io::Cursor::new(b"lorem\nnipsum\r\ndolor");
    let mut iter = cursor.lines().map(|r| r.unwrap());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}

fn test4() {
    println!("\ntest4\n");
    let cursor = io::Cursor::new(b"lorem\nnipsum\r\ndolor");
    let mut iter = cursor.lines().map(|r| r.unwrap());
    for i in iter {
        println!("{:?}", i);
    }
}
