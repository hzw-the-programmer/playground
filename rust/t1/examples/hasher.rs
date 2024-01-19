use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    let mut hasher = DefaultHasher::new();
    // let n: i32 = "Cool!"; // &str
    // let n: i32 = b"Cool!"; // &[u8; 5]
    // let n: i32 = b"Cool!"[..]; // [u8]
    // let a = b"Cool!"[..];
    // let n: i32 = &b"Cool!"[..]; // &[u8];
    // hasher.write("Cool!");
    hasher.write(b"Cool!");
    // hasher.write(b"Cool!"[..]);
    hasher.write(&b"Cool!"[..]);

    println!("Hash is {:x}!", hasher.finish());
}

fn test2() {
    let mut hasher = DefaultHasher::new();
    let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    // let n: i32 = data;
    // let n: i32 = &data;
    // let n: i32 = &data[..];
    // let n: i32 = data[..];
    // let a = data[..];
    hasher.write(&data);

    println!("Hash is {:x}!", hasher.finish());
}

fn test3() {
    let mut hasher = DefaultHasher::new();

    hasher.write_u32(1989);
    hasher.write_u8(11);
    hasher.write_u8(9);
    hasher.write(b"Huh?");

    println!("Hash is {:x}!", hasher.finish());
}
