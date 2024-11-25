pub fn test() {
    let tests = [test1, test2, test3, test4];
    for (i, test) in tests.iter().enumerate() {
        println!("\ntest{}: begin", i + 1);
        test();
        println!("test{}: end", i + 1);
    }
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

struct Foo(i32);

impl Foo {
    fn consume(self) {
        println!("Foo.consume");
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}

fn fn_once<F: FnOnce()>(f: F) {
    f();
}
