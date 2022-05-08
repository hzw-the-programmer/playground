#![allow(unused_variables, unused_assignments)]

use drop::Object;

fn main() {
    let mut o = Object { id: 1 };
    o = Object { id: 2 };
    println!("main finish")
}
