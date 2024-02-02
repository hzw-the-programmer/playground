use std::env::{self, current_dir};

fn main() {
    println!("{:?}", current_dir());
    println!("{:?}", dotenv::dotenv());
    for (k, v) in env::vars() {
        println!("{k}: {v}");
    }
    println!("value of SIZE is: {:?}", env::var("SIZE"));
}
