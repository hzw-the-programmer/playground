use std::cell::Cell;
use std::thread_local;

pub fn test() {
    test1();
}

struct Foo {
    i: Cell<i32>,
}

impl Foo {
    fn new(i: i32) -> Self {
        println!("Foo::new i={i}");
        Self { i: Cell::new(i) }
    }
}

thread_local! {
    static FOO: Foo = Foo::new(1);
}

fn test1() {
    println!("test1");
    FOO.with(|foo| {
        println!("Foo.with i={}", foo.i.get());
        foo.i.set(2);
    });
    FOO.with(|foo| {
        println!("Foo.with i={}", foo.i.get());
        foo.i.set(3);
    });
}
