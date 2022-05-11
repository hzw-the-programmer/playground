#![allow(dead_code)]

use drop::Object;
use std::ops::Deref;
use std::ops::DerefMut;

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

fn test1() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    let o = s.as_obj();
    println!("{:?}", o);
}

fn test2() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let mut s = S { f1, f2 };

    let o = s.as_mut_obj();
    println!("{:?}", o);
}

fn test3() {}

fn pro_obj(obj: &Object) {
    println!("{:?}", obj);
}

struct S {
    f1: Object,
    f2: Object,
}

impl S {
    fn as_obj(&self) -> &Object {
        println!("S::as_obj");
        self
    }

    fn as_mut_obj(&mut self) -> &mut Object {
        println!("S::as_mut_obj");
        self
    }
}

impl Drop for S {
    fn drop(&mut self) {
        println!("S dropped");
    }
}

impl Deref for S {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        println!("S::deref");
        &self.f2
    }
}

impl DerefMut for S {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("S::deref_mut");
        &mut self.f1
    }
}
