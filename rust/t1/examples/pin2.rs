struct Bar {
    id: i32,
    ptr: *const i32,
}

fn consume_bar(b: Bar) {
    println!("{:p}", &b.id);
    println!("{:p}", b.ptr);
}

fn main() {
    let mut b = Bar {
        id: 1,
        ptr: std::ptr::null(),
    };

    b.ptr = &b.id;

    println!("{:p}", &b.id);
    println!("{:p}", b.ptr);

    consume_bar(b);
}
