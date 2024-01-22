fn main() {
    test1();
}

struct Bar {
    id: i32,
    ptr: *const i32,
}

impl Bar {
    fn new(id: i32) -> Bar {
        let mut b = Bar {
            id,
            ptr: std::ptr::null(),
        };
        b.ptr = &b.id;
        println!("Bar.new");
        println!("id addr  : {:p}", &b.id);
        println!("ptr value: {:p}", b.ptr);
        b
    }
}

fn test1() {
    print!("\ntest1\n\n");
    let mut v = vec![];
    println!("len={}, capacity={}", v.len(), v.capacity());

    for id in 0..3 {
        let b = Bar::new(1);
        println!("before push");
        println!("id addr  : {:p}", &b.id);
        println!("ptr value: {:p}", b.ptr);
        v.push(b);
    }

    for b in &v {
        println!("id addr  : {:p}", &b.id);
        println!("ptr value: {:p}", b.ptr);
    }

    for b in v {
        println!("id addr  : {:p}", &b.id);
        println!("ptr value: {:p}", b.ptr);
    }
}
