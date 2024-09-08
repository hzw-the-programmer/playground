#![allow(dead_code)]

mod test1;

mod ref_val_1;
mod ref_val_2;

use ref_val_2 as test;

fn main() {
    test::test();
}
