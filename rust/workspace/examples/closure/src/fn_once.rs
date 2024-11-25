pub fn test() {
    // let tests = [test1, test2, test3, test4];
    // for (i, test) in tests.iter().enumerate() {
    //     println!("\ntest{}: begin", i + 1);
    //     test();
    //     println!("test{}: end", i + 1);
    // }

    // test5();
    // test6();
    // test7();
    // test8();
    test9();
}

fn test1() {
    let foo = Foo(1);
    let c = || {
        println!("closure enter");
        foo.consume();
        println!("closure leave");
    };
    // expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
    // let c: &dyn Fn() = &c;
    fn_once(c);
    println!("test1 leave");
}

fn test2() {
    let foo = Foo(1);

    struct C {
        foo: Foo,
    }
    impl C {
        fn call(self) {
            println!("closure enter");
            self.foo.consume();
            println!("closure leave");
        }
    }
    fn f1(c: C) {
        c.call();
    }

    let c = C { foo };
    f1(c);

    println!("test2 leave");
}

fn test3() {}

fn test4() {
    let foo = Foo(1);

    struct C {
        foo: Foo,
    }
    impl C {
        fn call(mut self) {
            println!("closure enter");
            self.foo.0 = 2;
            self.foo.consume();
            println!("closure leave");
        }
    }
    fn f1(c: C) {
        c.call();
    }

    let c = C { foo };
    f1(c);

    println!("test4 leave");
}

fn test5() {
    let f = Foo(0);
    {
        let c = || {
            println!("closure begin");
            f.byref();
            println!("closure end");
        };
        c();
    }
    println!("finish");
}

fn test6() {
    let f = Foo(0);
    {
        let c = move || {
            println!("closure begin");
            f.byref();
            println!("closure end");
        };
        c();
    }
    println!("finish");
}

fn test7() {
    let f = Foo(0);
    {
        let c = || {
            println!("closure begin");
            f.consume();
            println!("closure end");
        };
        c();
    }
    println!("finish");
}

fn test8() {
    let b = Bar {
        f1: Foo(0),
        f2: Foo(1),
    };
    b.f1.consume();
    // b.f1.byref();
    b.f2.byref();
    println!("finish");
}

fn test9() {
    let b = Bar {
        f1: Foo(1),
        f2: Foo(2),
    };
    let Bar { f1, .. } = b;
    f1.consume();
    b.f2.byref();
    println!("finish");
}

struct Foo(i32);

impl Foo {
    fn consume(self) {
        println!("Foo::consume {}", self.0);
    }

    fn byref(&self) {
        println!("Foo::byref {}", self.0);
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}

struct Bar {
    f1: Foo,
    f2: Foo,
}

// impl Drop for Bar {
//     fn drop(&mut self) {
//         println!("Bar drop");
//     }
// }

fn fn_once<F: FnOnce()>(f: F) {
    f();
}
