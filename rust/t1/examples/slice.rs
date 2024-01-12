use std::mem;

fn main() {
    let s = String::from("hello");
    let s1 = &s[..2];
    println!("{}", s1.len());
    println!("{}", mem::size_of::<&str>());
    println!("{}", mem::size_of::<Foo>());
    println!("{}", mem::size_of::<&Foo>());
    println!("{}", mem::size_of::<&[u8]>());
}

struct Foo {
    n1: u8,
    n2: u8,
}
