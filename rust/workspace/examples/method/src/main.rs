#![allow(unused_mut)]
#![allow(dead_code)]

mod test1;
mod test2;

use test2 as test;

fn main() {
    test::test();
}
