fn main() {
    test1();
    test2();
    test3();
}

struct Foo {
    id: i32,
    ptr: *const i32,
}

fn test1() {
    print!("\ntest1\n\n");

    let mut f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    f.ptr = &f.id;

    println!("stack:");
    println!("{:016p}", &f.id);
    println!("{:016p}", f.ptr);

    let p = Box::new(f);
    println!("heap");
    println!("{:016p}", &p.id);
    println!("{:016p}", p.ptr);

    let f1 = *p;
    println!("stack:");
    println!("{:016p}", &f1.id);
    println!("{:016p}", f1.ptr);

    // let n = p;
}

fn test2() {
    print!("\ntest2\n\n");

    let f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    println!("stack:");
    println!("{:016p}", &f.id);
    println!("{:016p}", f.ptr);

    let mut p = Box::new(f);
    p.ptr = &p.id;
    println!("heap");
    println!("{:016p}", &p.id);
    println!("{:016p}", p.ptr);

    let f1 = *p;
    println!("stack:");
    println!("{:016p}", &f1.id);
    println!("{:016p}", f1.ptr);

    // let n = p;
}

fn test3() {
    print!("\ntest3\n\n");

    let f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    let mut p = Box::pin(f);
    p.ptr = &p.id;
    println!();
    println!("id  addr : {:p}", &p.id);
    println!("ptr addr : {:p}", &p.ptr);
    println!("ptr value: {:p}", p.ptr);

    // let n: i32 = p;
    // let n: i32 = p.as_mut();

    // let p1 = *p;
    // let p1 = *(p.as_mut());
    // println!();
    // println!("id  addr : {:p}", &p1.id);
    // println!("ptr addr : {:p}", &p1.ptr);
    // println!("ptr value: {:p}", p1.ptr);
}
