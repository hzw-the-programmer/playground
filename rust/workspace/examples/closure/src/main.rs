#![allow(dead_code)]

mod fn_mut;
mod fn_once;

use fn_once as test;

fn main() {
    test::test();
}
