extern "C" {
    fn foo();
    fn bar();
}

fn main() {
    println!("Hello, world!");
    unsafe {
        foo();
        bar();
    }
}
