use drop::Object;
use std::mem;

fn test0() {
    println!("{}", mem::size_of::<S>());
    let f1 = Box::new(Object { id: 1 });
    let f2 = Box::new(Object { id: 2 });
    let s = S { f1, f2 };
}

fn test1() {
    let f1 = Box::new(Object { id: 1 });
    let ptr = Box::into_raw(f1);
    unsafe {
        *ptr = Object { id: 2 };
        println!("after assignment");
        Box::from_raw(ptr);
        println!("after from_raw");
    };
}

fn test2() {}

fn test3() {}

fn test4() {}

fn test5() {}
fn test6() {}
fn test7() {}
fn test8() {}
fn test9() {}
fn test10() {}

struct S {
    f1: Box<Object>,
    f2: Box<Object>,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("S::drop");
    }
}

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2, test3, test4, test5, test6, test7, test8, test9,
        test10, // test11,
               // test12, test13, test14, test15, test16, test17, test18, test19, test20, test21, test22,
               // test23, test24,
    ];

    drop::tests(&tests);
}
