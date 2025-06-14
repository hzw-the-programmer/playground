pub fn test() {
    // test1();
    test2();
}

fn test1() {
    let b = Box::new(1);
    println!("b: {:p}", b);

    let r = Box::into_raw(b);
    println!("r: {:p}", r);

    let b = unsafe { Box::from_raw(r) };
    println!("b: {:p}", b);

    let o = Some(b);
    let o1 = &o as *const Option<Box<i32>>;
    let o2 = o.as_ref().unwrap() as *const Box<i32>;
    println!("o: {:p}", o1);
    println!("o: {:p}", o2);

    let n = o.as_ref();
    let n1 = &n as *const Option<&Box<i32>>;
    let n2 = n.unwrap() as *const Box<i32>;
    println!("n: {:p}", n1);
    println!("n: {:p}", n2);

    assert!(o1 as u64 == o2 as u64);
    assert!(o1 as u64 != n1 as u64);
    assert!(o1 as u64 == n2 as u64);

    let b = unsafe { Box::from_raw(r) };
    println!("b: {:p}", b);

    let p = Some(b);
    println!("p: {:p}", &p as *const Option<Box<i32>>);
    println!("p: {:p}", p.as_ref().unwrap() as *const Box<i32>);

    let _ = std::mem::ManuallyDrop::new(p);
}

fn test2() {
    let a = Some(Box::new(1));
    let pa = &a as *const Option<Box<i32>>;
    let pb = a.as_ref().unwrap() as *const Box<i32>;
    assert_eq!(pa as u64, pb as u64);

    let n1 = a.as_ref();
    let n2 = a.as_ref();
    let p1 = &n1 as *const Option<&Box<i32>>;
    let p2 = &n2 as *const Option<&Box<i32>>;
    let p3 = n1.unwrap() as *const Box<i32>;
    let p4 = n2.unwrap() as *const Box<i32>;
    assert_ne!(p1 as u64, p2 as u64);
    assert_eq!(p3 as u64, p4 as u64);
    assert_ne!(p1 as u64, p3 as u64);
    assert_ne!(p2 as u64, p4 as u64);
}
