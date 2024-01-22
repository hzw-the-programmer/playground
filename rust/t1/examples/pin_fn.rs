fn main() {
    test1();
    test2();
}

struct Bar {
    id: i32,
    ptr: *const i32,
}

fn consume_bar(b: Bar) {
    println!("id  addr : {:p}", &b.id);
    println!("ptr addr : {:p}", &b.ptr);
    println!("ptr value: {:p}", b.ptr);
}

fn test1() {
    print!("\ntest1\n\n");
    let mut b = Bar {
        id: 1,
        ptr: std::ptr::null(),
    };

    b.ptr = &b.id;

    println!("id  addr : {:p}", &b.id);
    println!("ptr addr : {:p}", &b.ptr);
    println!("ptr value: {:p}", b.ptr);

    consume_bar(b);
}

#[derive(Copy, Clone)]
struct Bar2 {
    id: i32,
    ptr: *mut i32,
}

fn consume_bar2(b: Bar2) {
    unsafe {
        *b.ptr = 2;
    }
    println!("{}", b.id);
}

fn test2() {
    print!("\ntest2\n\n");
    let mut b = Bar2 {
        id: 1,
        ptr: std::ptr::null_mut(),
    };

    b.ptr = &mut b.id;

    consume_bar2(b);
    println!("{}", b.id);
}
