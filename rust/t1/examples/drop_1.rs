use t1::Foo;

fn test_1() {
    println!("\ntest_1\n");
    let _bar = Bar {
        id: 1,
        foo1: Foo { id: 1 },
        foo2: Foo { id: 2 },
    };
}

fn test_2() {
    println!("\ntest_2\n");
    let bar = Bar2 {
        id: 1,
        foo1: Foo { id: 1 },
        foo2: Foo { id: 2 },
    };

    {
        let foo1 = bar.foo1;
    }

    println!("finish");
}

fn test_3() {
    println!("\ntest_3\n");
    let bar = Bar {
        id: 1,
        foo1: Foo { id: 1 },
        foo2: Foo { id: 2 },
    };

    {
        // let foo1 = bar.foo1;
        let foo1 = &bar.foo1;
    }

    println!("finish");
}

fn test_4() {
    println!("\ntest_4\n");
    let foo = Foo { id: 1 };
    let bar = Bar2 {
        id: 1,
        foo1: Foo { id: 2 },
        foo2: Foo { id: 3 },
    };
    let b = Bar3 { id: 1 };
}

fn test_5() {
    println!("\ntest_5\n");
    let foo = Foo { id: 1 };
    let mut bar = Bar {
        id: 1,
        foo1: Foo { id: 2 },
        foo2: Foo { id: 3 },
    };
    let f = Foo { id: 4 };
    bar = Bar {
        id: 3,
        foo1: Foo { id: 5 },
        foo2: Foo { id: 6 },
    };
    println!("finish");
}

fn main() {
    test_1();
    test_2();
    test_4();
    test_5();
}

struct Bar {
    id: u64,
    foo1: Foo,
    foo2: Foo,
}

impl Drop for Bar {
    fn drop(&mut self) {
        println!("Bar {} drop", self.id);
    }
}

struct Bar2 {
    id: u64,
    foo1: Foo,
    foo2: Foo,
}

struct Bar3 {
    id: u64,
}
