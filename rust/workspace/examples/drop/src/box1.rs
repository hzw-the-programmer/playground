pub fn test() {
    // test1();
    test2();
}

fn test1() {
    struct Bar(i32);
    impl Drop for Bar {
        fn drop(&mut self) {
            println!("Bar {} drop", self.0);
        }
    }
    struct Foo {
        i: i32,
        bar1: Bar,
        bar2: Bar,
    }
    // impl Drop for Foo {
    //     fn drop(&mut self) {
    //         println!("Foo {} drop", self.i);
    //     }
    // }

    let foo = Box::new(Foo {
        i: 1,
        bar1: Bar(1),
        bar2: { Bar(2) },
    });
    // let foo = Foo {
    //     i: 1,
    //     bar1: Bar(1),
    //     bar2: Bar(2),
    // };
    {
        let _bar = foo.bar1;
    }

    println!("leave test1");
}

fn test2() {
    struct Bar(i32);
    impl Drop for Bar {
        fn drop(&mut self) {
            println!("Bar {} drop", self.0);
        }
    }
    struct Foo {
        i: i32,
        bar1: Bar,
        bar2: Bar,
    }
    // impl Drop for Foo {
    //     fn drop(&mut self) {
    //         println!("Foo {} drop", self.i);
    //     }
    // }

    let _bar;
    {
        // let foo = Box::new(Foo {
        //     i: 1,
        //     bar1: Bar(1),
        //     bar2: { Bar(2) },
        // });
        let foo = Foo {
            i: 1,
            bar1: Bar(1),
            bar2: Bar(2),
        };
        _bar = foo.bar1;
    }

    println!("leave test2");
}
