pub fn test() {
    // test1();
    // test2();
    test3();
}

fn test1() {
    let b = Box::new(Bar {
        f1: unsafe { std::mem::uninitialized() },
        f2: Foo(2),
    });
    println!("leave");
}

fn test2() {
    let b = Box::new(Bar {
        f1: unsafe { std::mem::uninitialized() },
        f2: Foo(2),
    });
    {
        let f1 = b.f1;
        println!("leave block");
    }
    println!("leave");
}

fn test3() {
    let b = Box::new(Bar {
        f1: unsafe { std::mem::uninitialized() },
        f2: Foo(2),
    });
    {
        // let f1 = b.f1;
        let _ = std::mem::ManuallyDrop::new(b.f1);
        println!("leave block");
    }
    println!("leave");
}

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}

struct Bar {
    f1: Foo,
    f2: Foo,
}
