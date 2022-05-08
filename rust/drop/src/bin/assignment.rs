#![allow(unused_variables, unused_assignments)]

use drop::{Object, S};

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2,
        test3, // test4, test5, test6,
              // test7, test8, test9, test10, test11,
              // test12, test13, test14, test15, test16, test17, test18, test19, test20, test21, test22,
              // test23, test24,
    ];

    drop::tests(&tests);
}

fn test0() {
    let mut o = Object { id: 1 };
    o = Object { id: 2 };
    println!("finish")
}

fn test1() {
    let o = Object { id: 1 };
    let o = S {
        f1: Object { id: 2 },
        f2: Object { id: 3 },
    };
    println!("finish");
}

fn test2() {}

fn test3() {}
