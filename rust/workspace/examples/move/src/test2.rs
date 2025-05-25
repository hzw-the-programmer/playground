pub fn test() {
    // test1();
    test2();
}

fn test1() {
    let b = Bar::new(1, 2);
    {
        let f = b.f1;
    }
    println!("after block");
    {
        let f = b.f2;
    }
    println!("leave");
}

fn test2() {
    let b = std::rc::Rc::new(Bar::new(1, 2));
    // cannot move out of an `Rc`
    // let f = b.f1;
    println!("{}", b.f1.i);
}

struct Foo {
    i: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} droped", self.i);
    }
}

struct Bar {
    f1: Foo,
    f2: Foo,
}

impl Bar {
    fn new(i: i32, j: i32) -> Self {
        Self {
            f1: Foo { i },
            f2: Foo { i: j },
        }
    }
}
