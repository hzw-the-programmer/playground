// #![feature(thread_spawn_unchecked)]
#![allow(dead_code)]

mod test1;
mod thread_local_1;

// use test1 as test;
use thread_local_1 as test;

fn main() {
    test::test();
}
