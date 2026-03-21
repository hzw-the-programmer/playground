use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();

    println!("Random die roll: {}", rng.random_range(1..=6));
    println!("Random UUID: 0x{:X}", rng.random::<u128>());

    if rng.random() {
        println!("You got lucky!");
    }

    // get some random data:
    let mut data = [0u8; 8];
    rand::rng().fill_bytes(&mut data);
    println!("{:?}", data)
}
