#![allow(dead_code)]

use std::mem;

enum Foo {
    V1,
    V2(Box<i32>),
}

enum FooTwo<'a> {
    V1,
    V2(&'a i32),
}

enum Bar {
    V1,
    V2(*const i32),
}

pub fn test() {
    assert_eq!(mem::size_of::<Foo>(), 8);
    assert_eq!(mem::size_of::<FooTwo>(), 8);
    assert_eq!(mem::size_of::<Bar>(), 16);
}
