#![allow(dead_code)]

use drop::Object;
use std::ops::Deref;

fn main() {
    let tests: Vec<fn()> = vec![test0, test1, test2, test3];

    for (i, test) in tests.iter().enumerate() {
        println!("/*** test {} ***/", i);
        test();
        println!("");
    }
}

fn test0() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    pro_obj(&s);
}

fn test1() {}

fn test2() {}

fn test3() {}

fn pro_obj(obj: &Object) {
    println!("{:?}", obj);
}

struct S {
    f1: Object,
    f2: Object,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("S dropped");
    }
}

impl Deref for S {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        return &self.f2;
    }
}
