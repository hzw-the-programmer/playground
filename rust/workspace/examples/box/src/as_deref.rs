pub fn test() {
    test1();
}

fn test1() {
    let b = Box::new(1);
    println!("b         : {:p}", b);
    let r = Box::into_raw(b);
    println!("r         : {:p}", r);
    let b = unsafe { Box::from_raw(r) };
    println!("b         : {:p}", b);
    println!("&*b       : {:p}", &*b);
    let o = Some(b);
    println!("as_ref    : {:p}", o.as_ref().unwrap());
    println!("*as_ref   : {:p}", *o.as_ref().unwrap());
    println!("&**as_ref : {:p}", &**o.as_ref().unwrap());
    println!("as_deref  : {:p}", o.as_deref().unwrap());
    println!("unwrap    : {:p}", o.unwrap());
}
