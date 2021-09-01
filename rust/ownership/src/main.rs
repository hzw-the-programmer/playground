use std::task::Poll;

#[derive(Debug)]
struct H {}

impl Drop for H {
    fn drop(&mut self) {
        println!("droped");
    }
}

impl H {
    fn new() -> H {
        H {}
    }
}

fn greeting(s: String) {
    println!("{}", s);
}

fn greeting_reference(s: &String) {
    println!("{}", s);
}

fn fo<F, T>(f: F, h1: T, h2: T) 
where
    F: FnOnce(T),
{
    f(h1);
    //f(h2);
    println!("hzw");
}

fn main() {
    // t1();
    t2();
}

fn t1() {
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

    let h1 = H{};
    let h2 = H{};
    fo(|x| {1;}, h1, h2);
}

fn t2() {
    let h = H::new();
    println!("returned from H::new: {:?}", h);
}