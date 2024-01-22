use std::marker::PhantomPinned;

struct Bar {
    id: i32,
    ptr: *const i32,
    _pin: PhantomPinned,
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
        _pin: PhantomPinned,
    };

    b.ptr = &b.id;

    println!("id  addr : {:p}", &b.id);
    println!("ptr addr : {:p}", &b.ptr);
    println!("ptr value: {:p}", b.ptr);

    consume_bar(b);
}

fn main() {
    test1();
}
