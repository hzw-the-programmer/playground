use std::task::Poll;
use std::alloc::{alloc, Layout};

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
    // t2();
    // t3();
    // t4();
    // t5();
    // t6();
    // t7();
    // t8();
    // f9();
    // f10();
    f11();
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

fn t3() {
    let h = H{};
    let b = Box::new(h);
    println!("end");
}

fn t4() {
    let h = H{};
    let b = Box::new(h);
    let p = Box::into_raw(b);
}

fn t5() {
    let h = H{};
    let b = Box::new(h);
    let p = Box::into_raw(b);
    let b1 = unsafe { Box::from_raw(p) };
    println!("end");
}

fn t6() {
    let p = unsafe { alloc(Layout::new::<H>()) as *mut H };
}

fn t7() {
    let p = unsafe { alloc(Layout::new::<H>()) as *mut H };
    let b = unsafe { Box::from_raw(p) };
    println!("end");
}

#[derive(Debug)]
struct Foo {
    f1: i32,
}

fn t8() {
    let foo1 = Foo{f1: 1};
    let foo2 = foo1;
    // println!("{:?}", foo1);
}

#[derive(Debug, Copy, Clone)]
struct Bar {
    f1: i32,
}

fn f9() {
    let mut bar1 = Bar{ f1: 1};
    let bar2 = bar1;
    bar1.f1 += 1;
    println!("{:?}, {:?}", bar1, bar2);
}

fn f10() {
    let i = 23;
    let r1 = &i;
    let r2 = r1;
    println!("{}, {}", r1, r2);
}

fn f11() {
    let mut h = H{};
    h = H{};
    println!("f11");
}