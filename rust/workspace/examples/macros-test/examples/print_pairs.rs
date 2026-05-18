macro_rules! print_pairs {
    ($($key:expr => $value:expr),*) => {
        $(println!("{}: {}", $key, $value);)*
    };
}

fn main() {
    print_pairs!("k1" => "v1", "k2" => "v2");
}

/*
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    {
        ::std::io::_print(format_args!("{0}: {1}\n", "k1", "v1"));
    };
    {
        ::std::io::_print(format_args!("{0}: {1}\n", "k2", "v2"));
    };
}
*/
