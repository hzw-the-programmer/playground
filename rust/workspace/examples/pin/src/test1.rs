use core::pin::Pin;

pub fn test() {
    test1();
}

fn test1() {
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
