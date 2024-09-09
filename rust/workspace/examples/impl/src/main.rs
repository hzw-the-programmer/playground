#![allow(dead_code)]

mod test1;

mod ref_val_1;
mod ref_val_2;

mod by_ref_1;
mod by_ref_2;

mod by_val_1;
mod by_val_2;

use by_val_2 as test;

fn main() {
    test::test();
}
