#![allow(dead_code)]

use std::mem;

struct Foo<T: ?Sized> {
    ptr: *const T,
}

pub fn test() {
    assert_eq!(mem::size_of::<Foo<u8>>(), 8);
    assert_eq!(mem::size_of::<Foo<[u8]>>(), 16);
}
