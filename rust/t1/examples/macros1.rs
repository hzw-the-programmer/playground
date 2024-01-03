use t1::Hello;

struct Pancakes;

impl Hello for Pancakes {
    fn hello() {
        println!("Hello! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello();
}
