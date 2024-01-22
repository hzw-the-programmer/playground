fn main() {
    test1();
    test2();
    test3();
}

// #[derive(Copy, Clone)]
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
    println!();
    println!("id  addr : {:p}", &f.id);
    println!("ptr addr : {:p}", &f.ptr);
    println!("ptr value: {:p}", f.ptr);

    let p = Box::new(f);
    println!();
    println!("id  addr : {:p}", &p.id);
    println!("ptr addr : {:p}", &p.ptr);
    println!("ptr value: {:p}", p.ptr);

    let f1 = *p;
    println!();
    println!("id  addr : {:p}", &f1.id);
    println!("ptr addr : {:p}", &f1.ptr);
    println!("ptr value: {:p}", f1.ptr);

    // println!();
    // println!("id  addr : {:p}", &p.id);
    // println!("ptr addr : {:p}", &p.ptr);
    // println!("ptr value: {:p}", p.ptr);
}

fn test2() {
    print!("\ntest2\n\n");

    let f = Foo {
        id: 1,
        ptr: std::ptr::null(),
    };
    let mut p = Box::new(f);
    p.ptr = &p.id;
    println!();
    println!("id  addr : {:p}", &p.id);
    println!("ptr addr : {:p}", &p.ptr);
    println!("ptr value: {:p}", p.ptr);

    let f1 = *p;
    println!();
    println!("id  addr : {:p}", &f1.id);
    println!("ptr addr : {:p}", &f1.ptr);
    println!("ptr value: {:p}", f1.ptr);
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

    let p1 = *p;
    // let p1 = *(p.as_mut());
    println!();
    println!("id  addr : {:p}", &p1.id);
    println!("ptr addr : {:p}", &p1.ptr);
    println!("ptr value: {:p}", p1.ptr);
}
