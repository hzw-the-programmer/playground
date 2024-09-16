pub fn test() {
    let tests = [test1, test2, test3, test4, test5];
    for (i, test) in tests.iter().enumerate() {
        println!("\ntest{}: begin", i + 1);
        test();
        println!("test{}: end", i + 1);
    }
}

fn test1() {
    let c = |x| x;
    println!("{}", c("hi closure!"));
    // println!("{}", c(2));
}

fn test2() {
    struct C;
    impl C {
        fn call(&self, p: &'static str) -> &'static str {
            p
        }
    }

    let c = C;
    println!("{}", c.call("hi closure!"));
}

fn test3() {
    println!("test3: enter");

    let foo = Foo(1);
    let c = || {
        println!("closure: enter");
        foo.consume();
        println!("closure: leave");
    };

    by_val(c);

    println!("test3: leave");
}

fn test4() {
    println!("test4: enter");

    let mut foo = Foo(1);
    let c = || {
        println!("closure: enter");
        foo.0 = 2;
        println!("closure: leave");
    };

    by_val_2(c);
    // c();

    println!("test4: leave");
}

fn test5() {
    println!("test5: enter");

    let mut foo = Foo(1);
    let mut c = || {
        println!("closure: enter");
        foo.0 = 2;
        println!("closure: leave");
    };

    by_ref_2(&mut c);
    c();

    println!("test5: leave");
}

fn by_val<F: FnOnce()>(f: F) {
    println!("by_val: enter");
    f();
    // f();
    println!("by_val: leave");
}

// fn by_ref<F: FnOnce()>(f: &F) {
//     f();
// }

fn by_val_2<F: FnMut()>(mut f: F) {
    println!("by_val_2: enter");
    f();
    f();
    println!("by_val_2: leave");
}

fn by_ref_2<F: FnMut()>(f: &mut F) {
    println!("by_ref_2: enter");
    f();
    f();
    println!("by_ref_2: leave");
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
