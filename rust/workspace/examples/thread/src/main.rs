#![feature(thread_spawn_unchecked)]

mod test1;

use test1 as test;

fn main() {
    test::test();
}
