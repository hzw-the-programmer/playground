pub fn test() {
    // test1();
    // test2();
    // test3();
    // test4();
    test5();
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

fn test4() {
    let mut x = std::mem::MaybeUninit::<&i32>::uninit();
    // expected `i32`, found `MaybeUninit<&i32>`
    // let i: i32 = x;
    x.write(&0);
    let x = unsafe { x.assume_init() };
    // expected `i32`, found `&i32`
    // let i: i32 = x;

    let b = Box::new(Bar {
        f1: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        f2: Foo(2),
    });
    println!("leave");
}

fn test5() {
    let b = Box::new(Bar {
        f1: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        f2: Foo(2),
    });
    let _ = std::mem::ManuallyDrop::new(b.f1);
    println!("leave");
}

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        if self.0 == 0 {
            panic!("id is zero");
        }
        println!("Foo {} drop", self.0);
    }
}

struct Bar {
    f1: Foo,
    f2: Foo,
}
