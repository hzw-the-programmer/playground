// macro_rules! test {
//     ($($n:expr),+) => {
//         $(
//             println!("\ntest{}: begin", $n);
//             test1();
//             println!("test{}: end", $n);
//         )+
//     };
// }

pub fn test() {
    // test!(1, 2);
    let tests = [test1, test2, test3, test4];
    for (i, test) in tests.iter().enumerate() {
        println!("\ntest{}: begin", i + 1);
        test();
        println!("test{}: end", i + 1);
    }
}

fn test1() {
    let mut foo = Foo(1);
    let c = || {
        println!("closure: enter");
        foo.0 = 2;
        println!("closure: leave");
    };
    // expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
    // let c: &dyn Fn() = &c;
    fn_mut(c);
    println!("test1 finish");
}

fn test2() {
    let mut foo = Foo(1);
    let c = move || {
        println!("closure: enter");
        foo.0 = 2;
        println!("closure: leave");
    };
    // expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
    // let c: &dyn Fn() = &c;
    fn_mut(c);
    println!("test2 finish");
}

fn test3() {
    let mut foo = Foo(1);

    struct C<'a> {
        foo: &'a mut Foo,
    }
    impl<'a> C<'a> {
        fn call(&mut self) {
            println!("closure: enter");
            self.foo.0 = 2;
            println!("closure: leave");
        }
    }
    fn f1(mut c: C) {
        c.call();
        c.call();
    }

    let c = C { foo: &mut foo };
    f1(c);

    println!("test3 finish");
}

fn test4() {
    let foo = Foo(1);

    struct C {
        foo: Foo,
    }
    impl C {
        fn call(&mut self) {
            println!("closure: enter");
            self.foo.0 = 2;
            println!("closure: leave");
        }
    }
    fn f1(mut c: C) {
        c.call();
        c.call();
    }

    let c = C { foo };
    f1(c);

    println!("test4 finish");
}

struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}

fn fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}
