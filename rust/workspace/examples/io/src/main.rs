#![feature(cursor_remaining)]
#![allow(dead_code)]

mod buf_read;
mod cursor;

// use cursor as test;
use buf_read as test;

fn main() {
    test::test();
}
