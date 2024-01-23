use std::ops::Deref;
use std::ops::DerefMut;

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

impl Deref for Foo {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.id
    }
}

impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("deref_mut");
        &mut self.id
    }
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    let f = Foo { id: 1 };
    {
        let p = std::pin::Pin::new(&f);
        // let ff = *p;
    }
    println!("finish");
}

fn test2() {
    print!("\ntest2\n\n");
    let mut f = Foo { id: 1 };
    let n: i32 = *f;
    println!("**");
    *f = 2;
    println!("**");
    println!("{}", *f);
    println!("finish");
}

#[derive(Debug)]
struct Foo1 {
    id: i32,
}

impl Foo1 {
    fn work(&self) {
        println!("Foo1.work");
    }

    fn work_mut(&mut self) {
        println!("Foo1.work_mut");
    }
}

struct Bar {
    foo: Foo1,
}

impl Deref for Bar {
    type Target = Foo1;

    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.foo
    }
}

impl DerefMut for Bar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("deref_mut");
        &mut self.foo
    }
}

fn test3() {
    print!("\ntest3\n\n");
    let mut b = Bar {
        foo: Foo1 { id: 1 },
    };
    println!("{:?}", *b);
    b.work();
    b.work_mut();
    // let f = *b;
    // let f = b.foo;
    // let f1 = b.foo;
    *b = Foo1 { id: 2 };
    println!("finish");
}

fn test4() {
    print!("\ntest4\n\n");
    let b = Bar {
        foo: Foo1 { id: 1 },
    };
    let bp = &b;
    // let b1 = *bp;
    let b1 = b;
    println!("finish");
}
