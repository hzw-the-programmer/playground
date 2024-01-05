use std::io::{Result, Write};

// impl<A: Allocator> Write for Vec<u8, A>
// impl<T, U, A, const N: usize> PartialEq<[U; N]> for Vec<T, A>

fn main() -> Result<()> {
    let mut v = Vec::new();

    println!("len={},cap={}", v.len(), v.capacity());

    write!(v, "hello")?;
    println!("len={},cap={}", v.len(), v.capacity());
    // let n: i32 = b"hello";
    // assert_eq!(b"hello", v);
    assert_eq!(v, b"hello");

    write!(v, " {}!", "world")?;
    println!("len={},cap={}", v.len(), v.capacity());
    assert_eq!(v, b"hello world!");

    Ok(())
}
