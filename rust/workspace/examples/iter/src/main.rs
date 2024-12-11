#![allow(dead_code)]
#![feature(iter_map_windows)]
#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]

mod iter;

use iter as test;

fn main() {
    test::test();
}
