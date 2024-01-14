use std::mem;

fn main() {
    let s = String::from("hello");
    let s1 = &s[..2];
    println!("{}", s1.len());
    println!("{}", mem::size_of::<&str>());
    println!("{}", mem::size_of::<Foo>());
    println!("{}", mem::size_of::<&Foo>());
    println!("{}", mem::size_of::<&[u8]>());

    test2();
    test3();
    test4();
}

struct Foo {
    n1: u8,
    n2: u8,
}

fn test2() {
    let a = [1, 2, 3];
    // let n: i32 = a;
    let b = &[1, 2, 3];
    // let n: i32 = b;
    let e = unsafe { a.get_unchecked(0) };
    println!("{}", e);

    let c = &[1, 2, 3][..];
    // let n: i32 = c;
}

fn test3() {
    let v = vec![1, 2, 3];
    // let n: i32 = v;
    // let n: i32 = &v;
    // let n: i32 = &v[..];
}

fn test4() {
    let s = String::from("hello world!");
    // let n: i32 = s;
    // let n: i32 = &s;
    // let n: i32 = &s[..];
}
