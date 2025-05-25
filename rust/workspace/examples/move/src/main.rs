#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

mod test1;
mod test2;
mod test3;

use test3 as test;

fn main() {
    test::test();
}
