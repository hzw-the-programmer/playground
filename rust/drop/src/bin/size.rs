use std::mem;

fn main() {
    // stack
    println!("size_of(String) == {}", mem::size_of::<String>());
    println!("size_of(Vec<i32>) == {}", mem::size_of::<Vec<i32>>());

    // stack ptr to stack
    println!("size_of(&String) == {}", mem::size_of::<&String>());
    println!("size_of(&Vec<i32>) == {}", mem::size_of::<&Vec<i32>>());
    println!("size_of(&&dyn H) == {}", mem::size_of::<&&dyn H>());

    // stack ptr to heap
    println!("size_of(Box<String>) == {}", mem::size_of::<Box<String>>());
    println!(
        "size_of(Box<Vec<i32>>) == {}",
        mem::size_of::<Box<Vec<i32>>>()
    );
    println!("size_of(Box<&dyn H>) == {}", mem::size_of::<Box<&dyn H>>());

    // println!("size_of(str) == {}", mem::size_of::<str>());
    // println!("size_of([u8]) == {}", mem::size_of::<[u8]>());

    // stack
    println!("size_of(&str) == {}", mem::size_of::<&str>());
    println!("size_of(&[u8]) == {}", mem::size_of::<&[u8]>());
    println!("size_of(&dyn Drop) == {}", mem::size_of::<&dyn Drop>());
    println!("size_of(&dyn H) == {}", mem::size_of::<&dyn H>());
}

trait H {
    fn h(&self);
}
