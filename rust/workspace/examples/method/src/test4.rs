use core::ops::Deref;

struct Foo;

impl Foo {
    fn f1(&self) {
        println!("Foo::f1");
    }
}

trait Trait1 {
    fn f1(&self);
    fn f2(&self);
}

impl Trait1 for Foo {
    fn f1(&self) {
        println!("Trait1::f1");
    }

    fn f2(&self) {
        println!("Trait1::f2");
    }
}

impl Deref for Foo {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &Bar
    }
}

struct Bar;

impl Bar {
    fn f1(&self) {
        println!("Bar::f1");
    }

    fn f2(&self) {
        println!("Bar::f2");
    }

    fn f3(&self) {
        println!("Bar::f3");
    }
}

trait Trait2 {
    fn f4(&self);
}

impl Trait2 for Bar {
    fn f4(&self) {
        println!("Trait2::f4");
    }
}

impl Deref for Bar {
    type Target = Baz;

    fn deref(&self) -> &Self::Target {
        &Baz
    }
}

struct Baz;

impl Baz {
    fn f5(&self) {
        println!("Baz::f5");
    }
}

pub fn test() {
    let foo = Foo;
    foo.f1();
    foo.f2();
    foo.f3();
    foo.f4();
    foo.f5();
}
