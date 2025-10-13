#![allow(dead_code)]
#![allow(unused_variables)]

// mod test1;
mod current_thread;

// use test1 as test;
use current_thread as test;

fn main() {
    test::test();
}
