#![allow(dead_code, array_into_iter)]

use drop::Object;

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2, test3, test4, test5, test6, test7, test8, test9, test10, test11,
        test12, test13, test14, test15, test16, test17, test18, test19, test20, test21, test22,
        test23, test24,
    ];

    drop::tests(&tests);
}

fn test0() {
    let objs = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];

    for obj in objs {
        println!("{:?}", obj);
    }

    println!("test 0 end");
}

fn test1() {
    let objs = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];

    for obj in objs {
        if obj.id == 2 {
            break;
        }
        println!("{:?}", obj);
    }

    println!("test 1 end");
}

fn test2() {
    let objs = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];

    for obj in &objs {
        println!("{:?}", obj);
    }

    println!("test 2 end");
}

fn test3() {
    let objs = vec![0, 1, 2, 3];

    let mut i = 0;
    for obj in objs {
        assert_eq!(i, obj);
        i += 1;
    }
}

fn test4() {
    let objs = vec![0, 1, 2, 3];

    let mut i = 0;
    for obj in &objs {
        assert_eq!(&i, obj);
        i += 1;
    }
}

fn test5() {
    let objs = vec![0, 1, 2, 3];

    let mut i = 0;
    for &obj in &objs {
        assert_eq!(i, obj);
        i += 1;
    }
}

fn test6() {
    let a = [1, 2, 3];
    let m = a.iter().map(|e| {
        //let i: i32 = e;
        e * 2
    });
    for i in m {
        let i: i32 = i;
        println!("{:?}", i);
    }
}

fn test7() {
    let a = [
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let m = a.iter().map(|e| Object { id: e.id + 1 });
    for e in m {
        println!("{:?}", e);
    }
    //let b: Vec<Object> = m.collect();
    println!("{:?}", a);
    //println!("{:?}", b);
}

fn test8() {
    let a = [
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let m = a.into_iter().map(|e| {
        //let i: i32 = e;
        Object { id: e.id + 1 }
    });
    for e in m {
        println!("{:?}", e);
    }
    //let b: Vec<Object> = m.collect();
    println!("{:?}", a);
    //println!("{:?}", b);
}

fn test9() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let m = a.into_iter().map(|e| {
        //let i: i32 = e;
        Object { id: e.id + 1 }
    });
    /*
    for e in m {
        println!("{:?}", e);
    }
    */
    println!("before collect");
    let b: Vec<Object> = m.collect();
    println!("after collect");
    //println!("{:?}", a);
    println!("{:?}", b);
}

fn test10() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let m = a.iter().map(|e| {
        //let i: i32 = e;
        Object { id: e.id + 1 }
    });
    /*
    for e in m {
        println!("{:?}", e);
    }
    */
    println!("before collect");
    let b: Vec<Object> = m.collect();
    println!("after collect");
    println!("{:?}", a);
    println!("{:?}", b);
}

fn test11() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let m = a.into_iter().take(3);
    for e in m {
        println!("{:?}", e);
    }
    println!("test11 finish");
}

fn test12() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let m = a.iter().take(3);
    for e in m {
        println!("{:?}", e);
    }
    println!("test11 finish");
}

fn test13() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!("{}", a.iter().any(|o| o.id == 0));
}

fn test14() {
    let a = vec![0, 1, 2, 3];
    println!("{}", a.iter().any(|&o| o == 0));
}

fn test15() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!("{}", a.iter().count());
}

fn test16() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!("{}", a.into_iter().count());
}

fn test17() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!(
        "{}",
        a.into_iter().any(|o| {
            println!("{:?}", o);
            o.id == 2
        })
    );
    println!("finish");
}

fn test18() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!("{:?}", a.into_iter().last());
    println!("finish");
}

fn test19() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!("{:?}", a.iter().last());
    println!("finish");
}

fn test20() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!("{:?}", a.into_iter().nth(1));
    println!("finish");
}

fn test21() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    println!("{:?}", a.iter().nth(1));
    println!("finish");
}

fn test22() {
    let a = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let mut i = a.into_iter();
    println!("{:?}", i.nth(1));
    println!("finish");
}

fn test23() {
    let v = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let mut i = v.into_iter();
    println!("{}", i.any(|o| { o.id == 2 }));
    println!("finish");
}

fn test24() {
    let v = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];
    let mut i = v.into_iter();
    println!("{:?}", i.find(|o| { o.id == 2 }));
    println!("finish");
}

fn t1(objs: &[Object]) {
    for obj in objs {
        println!("{:?}", obj);
    }
}

/*
fn t2(objs: &[Object]) {
    for &obj in objs {
        println!("{:?}", obj);
    }
}
 */

fn largest(items: &[i32]) -> i32 {
    let mut largest = items[0];
    for item in items {
        if largest > *item {
            largest = *item;
        }
    }
    largest
}

fn largest_1(items: &[i32]) -> i32 {
    let mut largest = items[0];
    for &item in items {
        if largest > item {
            largest = item;
        }
    }
    largest
}
