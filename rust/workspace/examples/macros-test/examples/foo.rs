macro_rules! foo {
    () => {
        let x = 1;
        println!("{}", x);
    };
}

fn main() {
    let x = 42;
    foo!(); // 打印 1，外部的 x 不受影响
    println!("{}", x);
}

/*
output:
1
42
*/

/*
cargo expand --example foo

#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    let x = 42;
    let x = 1;
    {
        ::std::io::_print(format_args!("{0}\n", x));
    };
    {
        ::std::io::_print(format_args!("{0}\n", x));
    };
}
*/
