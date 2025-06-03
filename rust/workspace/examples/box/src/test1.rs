pub fn test() {
    test1();
}

fn test1() {
    let b = Box::new(1);
    println!("b: {:p}", b);

    let r = Box::into_raw(b);
    println!("r: {:p}", r);

    let b = unsafe { Box::from_raw(r) };
    println!("b: {:p}", b);

    let o = Some(b);
    println!("o: {:p}", &o as *const Option<Box<i32>>);
    println!("o: {:p}", o.as_ref().unwrap() as *const Box<i32>);
}
