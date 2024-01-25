use std::mem;

fn main() {
    println!("i32: {}", mem::size_of::<i32>());
    println!("&i32: {}", mem::size_of::<&i32>());
    println!("[i8; 10]: {}", mem::size_of::<[i8; 10]>());
    println!("[i16; 10]: {}", mem::size_of::<[i16; 10]>());
    println!("[i32; 10]: {}", mem::size_of::<[i32; 10]>());
    println!("Option<i8>: {}", mem::size_of::<Option<i8>>());
    println!("Option<i16>: {}", mem::size_of::<Option<i16>>());
    println!("Option<i32>: {}", mem::size_of::<Option<i32>>());
    println!("Option<i64>: {}", mem::size_of::<Option<i64>>());
    println!("Option<[i8; 10]>: {}", mem::size_of::<Option<[i8; 10]>>());
    println!("Option<[i16; 10]>: {}", mem::size_of::<Option<[i16; 10]>>());
    println!("Option<[i32; 10]>: {}", mem::size_of::<Option<[i32; 10]>>());
    println!("Option<&i32>: {}", mem::size_of::<Option<&i32>>());
    println!("&str: {}", mem::size_of::<&str>());
}

struct Foo {
    id: i32,
}
