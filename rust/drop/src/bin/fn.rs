use drop::Object;

fn test0() {
    let f = func1();
    f();
    f();
    println!("finish");
}

fn test1() {
    {
        let f = func1();
        f();
        f();
    }
    println!("finish");
}

fn test2() {
    let f = func2();
    f();
    println!("finish");
}
fn test3() {}
fn test4() {}
fn test5() {}
fn test6() {}
fn test7() {}
fn test8() {}
fn test9() {}
fn test10() {}

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2, test3, test4, test5, test6, test7, test8, test9,
        test10, // test11,
               // test12, test13, test14, test15, test16, test17, test18, test19, test20, test21, test22,
               // test23, test24,
    ];

    drop::tests(&tests);
}

fn func1() -> impl Fn() {
    let s = Object { id: 1 };
    move || {
        println!("{:?}", s);
    }
}

fn func2() -> impl FnOnce() {
    let s = Object { id: 1 };
    move || {
        s.consume();
        println!("after consume");
    }
}
