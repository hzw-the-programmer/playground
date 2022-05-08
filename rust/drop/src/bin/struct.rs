#![allow(unused_variables)]

use drop::Object;

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2, test3, test4, test5, test6, test7, test8, test9, test10, test11,
        test12, test13,
    ];

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

fn test8() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    func1(s.f2);
    // println!("{:?}", s);
    println!("{:?}", s.f1);
    // println!("{:?}", s.f2);
    println!("finish")
}

fn test9() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let mut s = S { f1, f2 };

    func1(s.f2);
    s.f2 = Object { id: 3 };
    println!("{:?}", s);
    println!("{:?}", s.f1);
    println!("{:?}", s.f2);
    println!("finish")
}

fn test10() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    func2(s);
    println!("finish")
}

fn test11() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    func3(s);
    // println!("{:?}", s);
    println!("finish")
}

fn test12() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    func4(s);
    // println!("{:?}", s);
    println!("finish")
}

fn test13() {}

#[derive(Debug)]
struct S {
    f1: Object,
    f2: Object,
}

fn func1(o: Object) {
    println!("func1: {:?}", o);
}

fn func2(S { f1, f2 }: S) {
    println!("func2: {:?} {:?}", f1, f2);
}

fn func3(S { f1, ref f2 }: S) {
    println!("func3: {:?} {:?}", f1, f2);
}

fn func4(S { ref f1, ref f2 }: S) {
    println!("func4: {:?} {:?}", f1, f2);
}
