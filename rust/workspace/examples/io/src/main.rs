#![feature(cursor_remaining)]
#![allow(dead_code)]

mod buf_read;
mod cursor;
mod cursor2;

use cursor2 as test;
// use buf_read as test;

fn main() {
    test::test();
}
