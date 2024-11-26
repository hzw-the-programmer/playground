pub fn test() {
    // test5();
    // test6();
    // test7();
    // test8();
    // test9();
    test10();
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

fn test10() {
    let f1 = Foo(1);
    let c = || {
        let f2 = Foo(2);
        f1.byref();
        let c = || {
            f2.byref();
        };
        c();
    };
    c();
    println!("finish");
}

struct Foo(i32);

impl Foo {
    fn consume(self) {
        println!("Foo({})::consume", self.0);
    }

    fn byref(&self) {
        println!("Foo({})::byref", self.0);
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
