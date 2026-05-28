trait Trait1 {
    fn f1(&self);
    fn f2(&self);
}

struct Foo;

impl Trait1 for Foo {
    fn f1(&self) {
        println!("Trait1::f1 Foo");
    }

    fn f2(&self) {
        println!("Trait1::f2 Foo");
    }
}

impl Trait1 for &Foo {
    fn f1(&self) {
        println!("Trait1::f1 &Foo");
    }

    fn f2(&self) {
        println!("Trait1::f2 &Foo");
    }
}

fn g<T: Trait1>(t: T) {
    t.f1();
    t.f2();
}

pub fn test() {
    let foo = Foo;
    g(foo);

    let rfoo = &Foo;
    g(rfoo);

    let rfoo = &Foo;
    rfoo.f1();
    rfoo.f2();
}
