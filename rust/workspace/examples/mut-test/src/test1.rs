use core::ops::{Deref, DerefMut};

pub fn test() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    test8();
}

fn test1() {
    println!("\nteset1: enter");
    let mut f = Foo { id: 1 };
    let f = Foo { id: 2 };
    println!("teset1: leave");
}

fn test2() {
    println!("\nteset2: enter");
    let mut f = Foo { id: 1 };
    f = Foo { id: 2 };
    println!("teset2: leave");
}

fn test3() {
    println!("\ntest3: enter");
    let f = &mut Foo { id: 1 };
    f.id = 2;
    (*f).id = 3;
    // f = &mut Foo { id: 3 };
    println!("test3: leave");
}

fn test4() {
    println!("\ntest4: enter");
    let mut f = &Foo { id: 1 };
    // f.id = 2;
    // (*f).id = 3;
    f = &Foo { id: 4 };
    // println!("{}", f.id);
    println!("test4: leave");
}

fn test5() {
    fn fn1(f: &mut Foo) {
        f.id = 2;
    }
    let mut foo = Foo { id: 1 };
    fn1(&mut foo);
    println!("leave");
}

fn test6() {
    fn fn1(mut f: Foo) {
        f.id = 2;
    }
    let foo = Foo { id: 1 };
    fn1(foo);
    println!("leave");
}

fn test7() {
    fn fn1(mut f: Pin<&mut Foo>) {
        f.id = 2;
    }
    let foo = Pin {
        ptr: &mut Foo { id: 1 },
    };
    fn1(foo);
    println!("leave");
}

fn test8() {
    fn fn1(i: &Foo) {}
    fn fn2(i: &u8) {}
    let b = Bar(Foo { id: 1 });
    // let i: u8 = *b;
    // let i: u8 = **b;
    // let i: u8 = &*b;
    let i: &u8 = &*b;
    // let i: &u8 = &b;
    // let i: &Foo = &b;
    // let i: &Bar = &b;
    // fn1(&b);
    // fn2(&b);
}

struct Foo {
    id: u8,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        println!("Foo::deref");
        &self.id
    }
}

impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("Foo::deref_mut");
        &mut self.id
    }
}

struct Pin<Ptr> {
    ptr: Ptr,
}

impl<Ptr: Deref> Deref for Pin<Ptr> {
    type Target = Ptr::Target;

    fn deref(&self) -> &Self::Target {
        println!("Pin::deref");
        // &self.ptr
        &*self.ptr
        // self.ptr // if Ptr is not &Foo, but Foo
    }
}

impl<Ptr: DerefMut> DerefMut for Pin<Ptr> {
    fn deref_mut(&mut self) -> &mut Ptr::Target {
        println!("Pin::deref_mut");
        // &mut self.ptr
        &mut *self.ptr
        // self.ptr // if Ptr is not &mut Foo, but Foo
    }
}

struct Bar(Foo);

impl Deref for Bar {
    type Target = Foo;
    fn deref(&self) -> &Self::Target {
        println!("Bar::deref");
        &self.0
    }
}
