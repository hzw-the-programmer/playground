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

fn main() {
    test_1();
    test_2();
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
