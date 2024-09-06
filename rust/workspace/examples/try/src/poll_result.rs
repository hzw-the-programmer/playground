#![allow(dead_code)]

use core::task::Poll;

pub fn test() {
    test1();
    test2();
    test3();
}

fn test1() {
    println!("\ntest1: enter");
    let r = f1();
    println!("{:?}", r);
    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");
    let r = f2();
    println!("{:?}", r);
    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");
    let r = f3();
    println!("{:?}", r);
    println!("test3: leave");
}

fn f1() -> Poll<Result<Foo, Bar>> {
    let r: Poll<Result<Foo, Bar>> = Poll::Pending;
    let r = r?;
    println!("f1: after ? {:?}", r);
    r.map(Ok)
}

fn f2() -> Poll<Result<Foo, Bar>> {
    let r: Poll<Result<Foo, Bar>> = Poll::Ready(Ok(Foo));
    let r = r?;
    println!("f2: after ? {:?}", r);
    r.map(Ok)
}

fn f3() -> Poll<Result<Foo, Bar>> {
    let r: Poll<Result<Foo, Bar>> = Poll::Ready(Err(Bar));
    let r = r?;
    println!("f3: after ? {:?}", r);
    r.map(Ok)
}

#[derive(Debug)]
struct Foo;
#[derive(Debug)]
struct Bar;
