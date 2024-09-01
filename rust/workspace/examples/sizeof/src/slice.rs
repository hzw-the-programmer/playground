#![allow(dead_code)]

use core::mem;

pub fn test() {
    // assert_eq!(mem::size_of::<[Foo]>(), 16);
    assert_eq!(mem::size_of::<&[Foo]>(), 16);
    assert_eq!(mem::size_of::<&&[Foo]>(), 8);
    assert_eq!(mem::size_of::<Box<[Foo]>>(), 16);
    assert_eq!(mem::size_of::<Box<&[Foo]>>(), 8);

    assert_eq!(mem::size_of::<*const [Foo]>(), 16);
}

struct Foo(i32);
