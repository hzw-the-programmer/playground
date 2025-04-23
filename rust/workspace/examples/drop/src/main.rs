#![allow(dead_code)]

// 0769-sound-generic-drop
// 0738-variance

// mod sneetch;
mod test1;
// mod zook;
mod box1;
mod test2;

// use test2 as test;
use box1 as test;

fn main() {
    test::test();
}
