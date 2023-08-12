fn test_1() {
    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

    for _foo in foos {
        println!("in loop");
    }

    // for _foo in foos {
    //     println!("in loop");
    // }
}

fn test_2() {
    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

    for _foo in &foos {
        println!("in loop");
    }
}

fn test_3() {
    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

    for foo in &mut foos {
        println!("in loop");
        foo.id += 1;
    }
}

fn test_4() {
    let ids = vec![1, 2, 3, 4];

    for id in ids {
        println!("id={id}");
    }

    // for id in ids {
    //     println!("id={id}");
    // }
}

fn test_5() {
    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let filtered = foos.iter().filter(|foo| return foo.id % 2 == 0);

    for foo in filtered {
        println!("Foo {} in loop", foo.id);
    }

    foos.push(Foo { id: 5 });
}

fn test_6() {
    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let filtered = foos.into_iter().filter(|foo| {
        let ret = foo.id % 2 == 0;
        println!("Foo {} filtered: {}", foo.id, ret);
        ret
    });

    println!("before loop");
    for foo in filtered {
        println!("Foo {} in loop", foo.id);
    }
    println!("after loop");
}

fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
    test_6();
}

struct Foo {
    id: u64,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} dropped", self.id);
    }
}
