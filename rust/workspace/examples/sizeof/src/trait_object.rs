#![allow(dead_code)]

use core::mem;

pub fn test() {
    assert_eq!(mem::size_of::<&dyn Foo>(), 16);
    assert_eq!(mem::size_of::<Box<dyn Foo>>(), 16);
}

trait Foo {
    fn m1(&self);
}
