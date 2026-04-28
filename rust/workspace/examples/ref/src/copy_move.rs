// #![allow(unused_variables)]
// #![allow(dead_code)]
#![allow(unused)]

fn test1() {
    let a = 10;
    let r1 = &a;
    let r2 = r1;
    let r3 = r1;
}

fn test2() {
    let mut a = 10;
    let r1 = &mut a;
    let r2 = r1;
    // error[E0382]: use of moved value: `r1`
    // move occurs because `r1` has type `&mut i32`, which does not implement the `Copy` trait
    // let r3 = r1;
}

pub fn test() {}
