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

fn test_6() {
    println!("\ntest_6\n");
    let b = Bar4 {
        id: 1,
        b1: Bar {
            id: 1,
            foo1: Foo { id: 1 },
            foo2: Foo { id: 2 },
        },
        b2: Bar2 {
            id: 2,
            foo1: Foo { id: 3 },
            foo2: Foo { id: 4 },
        },
    };
}

fn test_7() {
    println!("\ntest_7\n");
    let bar = Bar2 {
        id: 1,
        foo1: Foo { id: 1 },
        foo2: Foo { id: 2 },
    };
    let f = bar.into_inner();
    println!("finish");
}

fn test_8() {
    println!("\ntest_8\n");
    let bar = Bar5 {
        id: 1,
        foo: Foo { id: 1 },
        foos: vec![Foo { id: 2 }, Foo { id: 3 }],
    };
    let f = bar.into_inner();
    println!("finish");
}

fn test_9() {
    println!("\ntest_9\n");
    let bar = Bar5 {
        id: 1,
        foo: Foo { id: 1 },
        foos: vec![Foo { id: 2 }, Foo { id: 3 }],
    };
    let f = bar.into_inner_1();
    println!("finish");
}

fn test_10() {
    println!("\ntest_9\n");
    let bar = Bar5 {
        id: 1,
        foo: Foo { id: 1 },
        foos: vec![Foo { id: 2 }, Foo { id: 3 }],
    };
    let f = bar.into_inner_2();
    println!("finish");
}

fn test_11() {
    println!("\ntest_11\n");
    let x = 5;
    let y = Box::new(x);
    println!("x = {}, y = {}", x, y);

    let x = Bar3 { id: 1 };
    let y = Box::new(x);
    // println!("x = {:?}, y = {:?}", x, y);

    let x = Bar3 { id: 1 };
    let y = x;
    // println!("x = {:?}, y = {:?}", x, y);

    let x = Bar6 { id: 1 };
    let y = Box::new(x);
    println!("x = {:?}, y = {:?}", x, y);
}

fn main() {
    test_1();
    test_2();
    test_4();
    test_5();
    test_6();
    test_7();
    test_8();
    test_9();
    test_10();
    test_11();
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

// impl Bar {
//     fn into_inner(self) -> Foo {
//         self.foo1
//     }
// }

struct Bar2 {
    id: u64,
    foo1: Foo,
    foo2: Foo,
}

impl Bar2 {
    fn into_inner(self) -> Foo {
        self.foo1
    }
}

#[derive(Debug)]
struct Bar3 {
    id: u64,
}

struct Bar4 {
    id: u64,
    b1: Bar,
    b2: Bar2,
}

impl Drop for Bar4 {
    fn drop(&mut self) {
        println!("Bar4 {} drop", self.id);
    }
}

struct Bar5 {
    id: u64,
    foo: Foo,
    foos: Vec<Foo>,
}

impl Bar5 {
    fn into_inner(self) -> Foo {
        self.foo
    }

    fn into_inner_1(self) -> Vec<Foo> {
        self.foos
    }

    fn into_inner_2(mut self) -> Foo {
        // self.foos[0]
        self.foos.remove(0)
    }
}

#[derive(Debug, Copy, Clone)]
struct Bar6 {
    id: u64,
}
