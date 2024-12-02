#![allow(dead_code)]

mod test1;
mod test2;
mod test3;
mod test4;

// use test1 as test;
// use test3 as test;
use test4 as test;

fn main() {
    test::test();
}
