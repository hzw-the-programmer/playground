// #![feature(cursor_remaining)]
#![allow(dead_code)]

// mod buf_read;
// mod cursor;
// mod cursor2;
mod write;

// use cursor2 as test;
// use buf_read as test;
use write as test;

fn main() {
    test::test();
}
