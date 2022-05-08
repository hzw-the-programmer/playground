#![allow(unused_variables)]

use drop::Object;

fn main() {
    let tests: Vec<fn()> = vec![test0, test1, test2, test3, test4, test5, test6, test7];

    drop::tests(&tests);
}

fn test0() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };
    println!("finish");
}

fn test1() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let mut s = S { f1, f2 };

    s.f1 = Object { id: 3 };
    println!("finish");
}

fn test2() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    {
        let S { f1, f2 } = s;
        println!("{:?} {:?}", f1, f2);
    }
    // println!("{:?}", s);
    println!("finish");
}

fn test3() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    {
        let S { ref f1, f2 } = s;
        println!("{:?} {:?}", f1, f2);
    }
    // println!("{:?}", s);
    println!("finish");
}

fn test4() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    {
        let S { f1, ref f2 } = s;
        println!("{:?} {:?}", f1, f2);
    }
    // println!("{:?}", s);
    // println!("{:?}", s.f1);
    println!("{:?}", s.f2);
    println!("finish");
}

fn test5() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    {
        let S { ref f1, ref f2 } = s;
        println!("{:?} {:?}", f1, f2);
    }
    println!("{:?}", s);
    println!("finish");
}

fn test6() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    {
        let S { f1, f2: _ } = s;
        // println!("{:?} {:?}", f1, f2);
        println!("{:?}", f1);
    }
    // println!("{:?}", s);
    // println!("{:?}", s.f1);
    println!("{:?}", s.f2);
    println!("finish");
}

fn test7() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let mut s = S { f1, f2 };

    {
        let f3 = Object { id: 3 };
        let f4 = Object { id: 4 };
        s.f1 = f3;
        println!("after assignment")
    }
    println!("finish")
}

#[derive(Debug)]
struct S {
    f1: Object,
    f2: Object,
}
