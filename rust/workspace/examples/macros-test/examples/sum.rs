macro_rules! sum {
    ($e:expr) => { $e };
    ($e:expr, $($rest:expr),*) => {
        $e + sum!($($rest),*)
    };
}

fn main() {
    sum!(1, 2, 3); // 展开为 1 + (2 + (3))
}

/*
cargo expand --example sum

#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    1 + (2 + 3);
}
*/
