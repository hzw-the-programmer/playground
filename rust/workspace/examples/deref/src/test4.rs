#![allow(unused_variables, dead_code)]

use std::io::IoSliceMut;

pub fn test() {}

fn read_verctored(bufs: &mut [IoSliceMut<'_>]) {
    let buf = bufs
        .iter_mut()
        .find(|b| {
            // let i: i32 = b;
            !b.is_empty()
        })
        .map_or(&mut [][..], |b| {
            // let i: i32 = b;
            b
            // &mut *b
            // &mut **b
        });
    // let i: i32 = buf;
}
