#![allow(dead_code)]

pub fn test() {
    test1();
    test2();
}

fn test1() {
    println!("\ntest1: enter");

    // let mut v = Vec::new();
    let mut v = Vec::<&dyn Foo>::new();
    v.push(&Bar(1));
    v.push(&Baz(2));
    for i in v {
        i.m1();
    }

    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");

    let mut v = Vec::<Box<dyn Foo>>::new();
    v.push(Box::new(Bar(1)));
    // v.push(Box::new(&Baz(2)));
    v.push(Box::new(Baz(2)));
    for i in v {
        i.m1();
    }

    println!("test2: leave");
}

trait Foo {
    fn m1(&self);
}

struct Bar(i32);

impl Foo for Bar {
    fn m1(&self) {
        println!("Bar.m1");
    }
}

struct Baz(i32);

impl Foo for Baz {
    fn m1(&self) {
        println!("Baz.m1");
    }
}
