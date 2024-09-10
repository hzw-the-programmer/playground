#![feature(noop_waker)]

mod future_1;

use future_1 as test;

fn main() {
    test::test();
}
