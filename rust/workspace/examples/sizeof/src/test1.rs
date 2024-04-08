use std::{io::Write, mem};

pub fn test() {
    assert_eq!(mem::size_of::<&u8>(), 8);
    assert_eq!(mem::size_of::<*const u8>(), 8);
    assert_eq!(mem::size_of::<Box<u8>>(), 8);

    assert_eq!(mem::size_of::<&[u8]>(), 16);
    assert_eq!(mem::size_of::<*const [u8]>(), 16);
    assert_eq!(mem::size_of::<Box<[u8]>>(), 16);

    assert_eq!(mem::size_of::<&dyn Write>(), 16);
    assert_eq!(mem::size_of::<*const dyn Write>(), 16);
    assert_eq!(mem::size_of::<Box<dyn Write>>(), 16);
}
