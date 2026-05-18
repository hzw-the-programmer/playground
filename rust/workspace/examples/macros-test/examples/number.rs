macro_rules! number {
    (one) => {{ 1 }};
    ($x:expr) => {
        $x
    };
}

fn main() {
    number!(one); // 匹配第一条规则，得到 1
    number!(42); // 匹配第二条规则，得到 42
}

/*
cargo expand --example number

#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    { 1 };
    42;
}
*/
