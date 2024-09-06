#![feature(try_trait_v2)]
// rustup default nightly
#![feature(try_blocks)]

mod poll_result;

mod foo_bar;

mod poll_result_custom;
mod poll_result_custom_test;

use poll_result_custom_test as test;
// use poll_result as test;

fn main() {
    test::test();
}
