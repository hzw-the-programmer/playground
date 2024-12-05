#![allow(dead_code)]
#![allow(unused_variables)]

mod drop;
mod fn_1;
mod fn_2;
mod fn_mut;
mod fn_once;
mod move_test;

// use fn_2 as test;
// use fn_once as test;
// use drop as test;
use move_test as test;

fn main() {
    test::test();
}
