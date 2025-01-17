#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod test1;
mod test2;

use test2 as test;

fn main() {
    test::test();
}
