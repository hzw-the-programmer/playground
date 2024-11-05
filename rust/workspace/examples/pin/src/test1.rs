use core::ops::{Deref, DerefMut};
use core::pin::Pin;

pub fn test() {
    test1();
    test2();
    test3();
}

fn test1() {
    println!("\ntest1: enter");

    let mut foo = Foo;
    foo.f1();
    foo.f2();
    // no method named `f3` found for struct `Foo` in the current scope
    // foo.f3();
    // the trait bound `Foo: Deref` is not satisfied
    // Pin::new(foo).f3();
    // no method named `f3` found for struct `Pin<&Foo>` in the current scope
    // Pin::new(&foo).f3();
    Pin::new(&mut foo).f3();

    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");

    let mut bar = Bar(Foo);
    bar.f1();
    bar.f2();
    // no method named `f3` found for struct `Bar` in the current scope
    // bar.f3();
    // no method named `f3` found for struct `Pin<Bar>` in the current scope
    // Pin::new(bar).f3();
    // no method named `f4` found for struct `Pin<&mut Foo>` in the current scope
    // Pin::new(bar).as_mut().f4();
    Pin::new(bar).as_mut().f3();

    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");

    let mut foo = Foo;

    let pf = Pin::new(&mut foo);
    pf.f3();
    // move occurs because `pf` has type `Pin<&mut Foo>`, which does not implement the `Copy` trait
    // pf.f3();

    let mut pf = Pin::new(&mut foo);
    pf.as_mut().f3();
    pf.as_mut().f3();

    println!("test3: leave");
}

struct Foo;

impl Foo {
    fn f1(&mut self) {
        println!("Foo.f1(&mut self)");
    }
    fn f2(self: &mut Self) {
        println!("Foo.f2(self: &mut Self)");
    }
    fn f3(self: Pin<&mut Self>) {
        println!("Foo.f3(self: Pin<&mut Self>)");
    }
}

struct Bar(Foo);

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
