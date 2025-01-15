#![allow(dead_code)]

mod test1;
mod test2;
mod test3;
mod test4;
mod test5;

use test5 as test;

fn main() {
    test::test();
}
