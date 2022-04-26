use drop::{Object, S};

fn main() {
    let tests: Vec<fn()> = vec![test0, test1, test2, test3];

    for (i, test) in tests.iter().enumerate() {
        println!("/*** test {} ***/", i);
        test();
        println!("");
    }
}

fn test0() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let _s = S { f1, f2 };
}

fn test1() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let mut s = S { f1, f2 };

    s.f1 = Object { id: 3 };
}

fn test2() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    //let S { f1, f2 } = s;
    //println!("{:?} {:?}", f1, f2);
    //let S { ref f1, f2 } = s;
    //println!("{:?} {:?}", f1, f2);
    let S { ref f1, ref f2 } = s;
    println!("{:?} {:?}", f1, f2);
    let S { ref f1, f2: _ } = s;
    println!("{:?}", f1);
}

fn test3() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let mut s = S { f1, f2 };

    {
        let f3 = Object { id: 3 };
        let _f4 = Object { id: 4 };
        s.f1 = f3;
        println!("after assignment")
    }
    println!("end")
}
