#![allow(dead_code)]
#![feature(iter_map_windows)]
#![feature(iterator_try_collect)]

mod iter;

use iter as test;

fn main() {
    test::test();
}
