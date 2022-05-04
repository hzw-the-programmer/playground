use std::mem;

fn main() {
    println!("size_of(String) == {}", mem::size_of::<String>());
    println!("size_of(&String) == {}", mem::size_of::<&String>());
    //println!("size_of(str) == {}", mem::size_of::<str>());
    println!("size_of(&str) == {}", mem::size_of::<&str>());
    println!("size_of(Box<String>) == {}", mem::size_of::<Box<String>>());
    println!("size_of(&dyn Drop) == {}", mem::size_of::<&dyn Drop>());
    println!("size_of(&dyn H) == {}", mem::size_of::<&dyn H>());
    println!("size_of(Box<&dyn H>) == {}", mem::size_of::<Box<&dyn H>>());
}

trait H {
    fn h(&self);
}
