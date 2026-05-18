macro_rules! make_var {
    ($name:ident) => {{
        let $name = 10;
    }};
}

fn main() {
    let secret = 20;
    make_var!(secret);
    println!("{}", secret); // 可以访问，因为 $name 是在调用处解析
}

/*
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    let secret = 20;
    {
        let secret = 10;
    };
    {
        ::std::io::_print(format_args!("{0}\n", secret));
    };
}
*/
