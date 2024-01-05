use std::fmt;
use std::io::{Result, Write};

// impl<A: Allocator> Write for Vec<u8, A>
// impl<T, U, A, const N: usize> PartialEq<[U; N]> for Vec<T, A>

fn main() -> Result<()> {
    let mut v = Vec::new();
    // v.reserve(12);

    println!("len={},cap={}", v.len(), v.capacity());

    write!(v, "hello")?;
    println!("len={},cap={}", v.len(), v.capacity());
    // let n: i32 = b"hello";
    // assert_eq!(b"hello", v);
    assert_eq!(v, b"hello");

    write!(v, " {}!", "world")?;
    println!("len={},cap={}", v.len(), v.capacity());
    assert_eq!(v, b"hello world!");

    // write!(
    //     v,
    //     " I'm here: {:?}",
    //     Position {
    //         longitude: 121.48054,
    //         latitude: 31.23593
    //     }
    // );
    // println!("len={},cap={}", v.len(), v.capacity());
    // assert_eq!(v, b"hello world! I'm here: Position { longitude: 121.48054, latitude: 31.23593 }");
    write!(
        v,
        " I'm here: {}",
        Position {
            longitude: 121.48054,
            latitude: 31.23593
        }
    );
    println!("len={},cap={}", v.len(), v.capacity());
    assert_eq!(v, b"hello world! I'm here: (121.48054, 31.23593)");

    Ok(())
}

#[derive(Debug)]
struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}
