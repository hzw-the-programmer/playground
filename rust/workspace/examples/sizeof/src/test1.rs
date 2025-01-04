#![allow(dead_code)]

use std::{io::Write, mem};

pub fn test() {
    // test1();
    // test2();
    test3();
}

fn test1() {
    assert_eq!(mem::size_of::<&u8>(), 8);
    assert_eq!(mem::size_of::<*const u8>(), 8);
    assert_eq!(mem::size_of::<Box<u8>>(), 8);

    assert_eq!(mem::size_of::<&[u8]>(), 16);
    assert_eq!(mem::size_of::<*const [u8]>(), 16);
    assert_eq!(mem::size_of::<Box<[u8]>>(), 16);

    assert_eq!(mem::size_of::<&dyn Write>(), 16);
    assert_eq!(mem::size_of::<*const dyn Write>(), 16);
    assert_eq!(mem::size_of::<Box<dyn Write>>(), 16);

    assert_eq!(mem::size_of::<&str>(), 16);
    assert_eq!(mem::size_of::<*const str>(), 16);
    assert_eq!(mem::size_of::<Box<str>>(), 16);
}

fn test2() {
    // required by the implicit `Sized` requirement on this type parameter in `size_of`
    // consider relaxing the implicit `Sized` restriction
    // fn size_of<T>() -> usize {
    fn size_of<T: ?Sized>() -> usize {
        mem::size_of::<&T>()
    }

    assert_eq!(size_of::<u8>(), 8);
    assert_eq!(size_of::<[u8]>(), 16);
}

fn test3() {
    fn size_of<T>() -> usize {
        mem::size_of::<T>()
    }

    assert_eq!(size_of::<u8>(), 1);
    assert_eq!(size_of::<&u8>(), 8);
    assert_eq!(size_of::<&[u8]>(), 16);
}
