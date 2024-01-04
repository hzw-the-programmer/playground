use std::io::{self, Seek};
use t1::Foo;

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("\ntest1\n");
    let mut cursor = io::Cursor::new(Foo { id: 1 });
    println!("{}", cursor.position());
    cursor.set_position(1);
    println!("{}", cursor.position());
    // cursor.stream_position();
}

fn test2() {
    println!("\ntest2\n");
    // let mut cursor = io::Cursor::new(vec![1i32, 2, 3]);
    let mut cursor = io::Cursor::new(vec![1, 2, 3]);
    println!("{}", cursor.position());
    cursor.set_position(1);
    println!("{}", cursor.position());
    cursor.stream_position();
}
