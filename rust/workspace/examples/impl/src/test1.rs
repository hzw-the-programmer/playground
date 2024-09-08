pub fn test() {
    test1();
    test2();
    test3();
}

fn test1() {
    println!("\ntest1: enter");
    let f = Foo { id: 1 };
    f.fn_1();
    (&f).fn_1();
    (&f).fn_2();
    (&&f).fn_1();
    f.fn_2();
    // f.fn_1();
    println!("test1: leave");
}

fn test2() {
    println!("\ntest2: enter");
    let f = Foo { id: 1 };
    let r = &f;
    r.fn_1();
    r.fn_2();
    println!("test2: leave");
}

fn test3() {
    println!("\ntest3: enter");
    let r = &Foo { id: 1 };
    r.fn_1();
    r.fn_2();
    println!("test3: leave");
}

struct Foo {
    id: u8,
}

impl Foo {
    fn do_1(&self) {
        println!("Foo.do_1");
    }
}

// impl &Foo {
//     fn do_1(self) {
//         println!("&Foo.do_1");
//     }
// }

trait TraitOne {
    fn fn_1(&self);
    fn fn_2(self);
}

impl TraitOne for Foo {
    fn fn_1(&self) {
        println!("Foo.fn_1");
    }

    fn fn_2(self) {
        println!("Foo.fn_2");
    }
}

impl TraitOne for &Foo {
    fn fn_1(&self) {
        println!("&Foo.fn_1 begin");

        // let i: i32 = self;
        // self.fn_1(); // thread 'main' has overflowed its stack

        // let i: i32 = *self;
        // (*self).fn_1();

        // let i: i32 = **self;
        // (**self).fn_1();

        // let i: i32 = &**self;
        (&**self).fn_1();

        println!("&Foo.fn_1: end");
    }

    fn fn_2(self) {
        // let i: i32 = self;
        println!("&Foo.fn_2");
        // self.fn_2(); // thread 'main' has overflowed its stack
        // (*self).fn_2();
    }
}
