// cargo run --example vec_1

fn test_1() {
    println!();
    println!("****** test_1 ******");
    println!();

    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

    for _foo in foos {
        println!("in loop");
    }

    // for _foo in foos {
    //     println!("in loop");
    // }

    println!("finish");
}

fn test_2() {
    println!();
    println!("****** test_2 ******");
    println!();

    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

    for _foo in &foos {
        println!("in loop");
    }

    println!("finish");
}

fn test_3() {
    println!();
    println!("****** test_3 ******");
    println!();

    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

    for foo in &mut foos {
        println!("in loop");
        foo.id += 1;
    }

    println!("finish");
}

fn test_4() {
    println!();
    println!("****** test_4 ******");
    println!();

    let ids = vec![1, 2, 3, 4];

    for id in ids {
        println!("id={id}");
    }

    // for id in ids {
    //     println!("id={id}");
    // }

    println!("finish");
}

// fn filter<P>(self, predicate: P) -> Filter<Self, P>
// where
//     Self: Sized,
//     P: FnMut(&Self::Item) -> bool,

fn test_5() {
    println!();
    println!("****** test_5 ******");
    println!();

    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let filtered = foos.iter().filter(|foo| {
        // let i: i32 = foo;
        let ret = foo.id % 2 == 0;
        println!("Foo {} filtered: {}", foo.id, ret);
        ret
    });

    println!("before loop");
    for foo in filtered {
        println!("Foo {} in loop", foo.id);
    }
    println!("after loop");

    foos.push(Foo { id: 5 });

    println!("finish");
}

fn test_6() {
    println!();
    println!("****** test_6 ******");
    println!();

    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let filtered = foos.into_iter().filter(|foo| {
        // let i: i32 = foo;
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

// fn map<B, F>(self, f: F) -> Map<Self, F>
// where
//     Self: Sized,
//     F: FnMut(Self::Item) -> B,

fn test_7() {
    println!();
    println!("****** test_7 ******");
    println!();

    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.iter().map(|foo| {
        // let i: i32 = foo;
        println!("Foo {}", foo.id);
        foo.id
    });

    println!("before loop");
    for foo in iter {
        println!("Foo {} in loop", foo);
    }
    println!("after loop");

    foos.push(Foo { id: 5 });

    println!("finish");
}

fn test_8() {
    println!();
    println!("****** test_8 ******");
    println!();

    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.into_iter().map(|foo| {
        // let i: i32 = foo;
        println!("Foo {}", foo.id);
        foo.id
    });

    println!("before loop");
    for foo in iter {
        println!("Foo {} in loop", foo);
    }
    println!("after loop");
}

// fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
// where
//     Self: Sized,
//     P: FnMut(&Self::Item) -> bool,

fn test_9() {
    println!();
    println!("****** test_9 ******");
    println!();

    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.iter().skip_while(|foo| {
        // let i: i32 = foo;
        println!("Foo {} in predicate", foo.id);
        foo.id < 3
    });

    println!("before loop");
    for foo in iter {
        // let i: i32 = foo;
        println!("Foo {} in loop", foo.id);
    }
    println!("after loop");

    foos.push(Foo { id: 5 });

    println!("finish");
}

fn test_10() {
    println!();
    println!("****** test_10 ******");
    println!();

    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.into_iter().skip_while(|foo| {
        // let i: i32 = foo;
        println!("Foo {} in predicate", foo.id);
        foo.id < 3
    });

    println!("before loop");
    for foo in iter {
        println!("Foo {} in loop", foo.id);
    }
    println!("after loop");
}

// fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
// where
//     Self: Sized,
//     P: FnMut(&Self::Item) -> bool,

fn test_11() {
    println!();
    println!("****** test_11 ******");
    println!();

    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.iter().take_while(|foo| {
        // let i: i32 = foo;
        println!("Foo {} in predicate", foo.id);
        foo.id < 3
    });

    println!("before loop");
    for foo in iter {
        // let i: i32 = foo;
        println!("Foo {} in loop", foo.id);
    }
    println!("after loop");

    foos.push(Foo { id: 5 });

    println!("finish");
}

fn test_12() {
    println!();
    println!("****** test_12 ******");
    println!();

    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.into_iter().take_while(|foo| {
        // let i: i32 = foo;
        println!("Foo {} in predicate", foo.id);
        foo.id < 3
    });

    println!("before loop");
    for foo in iter {
        // let i: i32 = foo;
        println!("Foo {} in loop", foo.id);
    }
    println!("after loop");
}

// fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P>
// where
//     Self: Sized,
//     P: FnMut(Self::Item) -> Option<B>,

fn test_13() {
    println!();
    println!("****** test_13 ******");
    println!();

    let mut foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.iter().map_while(|foo| {
        // let i: i32 = foo;
        println!("Foo {} in predicate", foo.id);
        if foo.id < 3 {
            Some(foo.id)
        } else {
            None
        }
    });

    println!("before loop");
    for foo in iter {
        // let i: i32 = foo;
        println!("Foo {} in loop", foo);
    }
    println!("after loop");

    foos.push(Foo { id: 5 });

    println!("finish");
}

fn test_14() {
    println!();
    println!("****** test_14 ******");
    println!();

    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }, Foo { id: 4 }];

    let iter = foos.into_iter().map_while(|foo| {
        // let i: i32 = foo;
        println!("Foo {} in predicate", foo.id);
        if foo.id < 3 {
            Some(foo.id)
        } else {
            None
        }
    });

    println!("before loop");
    for foo in iter {
        // let i: i32 = foo;
        println!("Foo {} in loop", foo);
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
    test_7();
    test_8();
    test_9();
    test_10();
    test_11();
    test_12();
    test_13();
    test_14();
}

struct Foo {
    id: u64,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} dropped", self.id);
    }
}
