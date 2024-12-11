#![allow(dead_code)]
#![feature(iter_map_windows)]
#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(iter_partition_in_place)]

mod iter;

use iter as test;

fn main() {
    test::test();
}
