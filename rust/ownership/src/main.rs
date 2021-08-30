use std::task::Poll;

#[derive(Debug)]
struct H {}

impl Drop for H {
    fn drop(&mut self) {
        println!("droped");
    }
}

fn greeting(s: String) {
    println!("{}", s);
}

fn greeting_reference(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    // greeting(s);
    // greeting_reference(s);
    greeting_reference(&s);

    let s1 = String::from("hello world!");
    let p = Poll::Ready(s1);
    // println!("{}", s1);

    let h = H{};
    let p = Poll::Ready(h);
    // println!("{:?}", h);
    let p1 = p.map(|x| {
        println!("hahaha");
        1
    });
    // println!("{:?}", p);
    println!("hee");
}
