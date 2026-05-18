macro_rules! demo {
    () => {
        println!("hello")
    };
}

fn main() {
    demo!(); // 正确
    let _x = 0;
    demo!()
}

/*
cargo expand --example semicon

#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    {
        ::std::io::_print(format_args!("hello\n"));
    };
    let _x = 0;
    {
        ::std::io::_print(format_args!("hello\n"));
    }
}
*/
