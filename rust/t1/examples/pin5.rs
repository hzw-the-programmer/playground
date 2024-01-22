use std::pin::Pin;

fn main() {
    let mut val: u8 = 5;
    let mut pinned: Pin<&mut u8> = Pin::new(&mut val);
    println!("{}", pinned); // 5
    // pinned.as_mut().set(10);
    // pinned.set(10);
    *pinned = 10;
    println!("{}", pinned); // 10
}
