#![allow(dead_code)]
#![allow(unused_variables)]

mod fn_1;
mod fn_2;
mod fn_mut;
mod fn_once;

// use fn_2 as test;
use fn_once as test;

fn main() {
    test::test();
}
