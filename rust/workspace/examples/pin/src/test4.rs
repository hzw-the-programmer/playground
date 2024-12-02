use core::ops::{Deref, DerefMut};

pub fn test() {
    test1();
    test2();
}

fn test1() {
    let foo = Foo { ptr: Bar(Baz) };
    let _v = &*foo;
}

fn test2() {
    let mut foo = Foo { ptr: Bar(Baz) };
    let _v = &mut *foo;
}

struct Foo<PTR> {
    ptr: PTR,
}

impl<PTR: Deref> Deref for Foo<PTR> {
    type Target = PTR::Target;
    fn deref(&self) -> &Self::Target {
        println!("Foo::deref");
        &self.ptr
    }
}

impl<PTR: DerefMut> DerefMut for Foo<PTR> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("Foo::deref_mut");
        &mut self.ptr
    }
}

struct Bar(Baz);

impl Deref for Bar {
    type Target = Baz;
    fn deref(&self) -> &Self::Target {
        println!("Bar::deref");
        &self.0
    }
}

impl DerefMut for Bar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("Bar::deref_mut");
        &mut self.0
    }
}

struct Baz;
