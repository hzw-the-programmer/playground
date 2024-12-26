#![allow(dead_code)]

// 0769-sound-generic-drop
// 0738-variance

// mod sneetch;
mod test1;
// mod zook;
mod test2;
mod test3;

use test3 as test;

fn main() {
    test::test();
}
