struct Bar {
    id: i32,
    ptr: *const i32,
}

fn consume_bar(b: Bar) {
    println!("id  addr : {:p}", &b.id);
    println!("ptr addr : {:p}", &b.ptr);
    println!("ptr value: {:p}", b.ptr);
}

fn main() {
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
