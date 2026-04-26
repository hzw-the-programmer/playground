struct SelfRef {
    data: String,
    ptr: *const String,
}

impl SelfRef {
    fn new(data: String) -> Self {
        let mut s = Self {
            data,
            ptr: std::ptr::null(),
        };

        s.ptr = &s.data;

        println!("s     : {:p}", &s);
        println!("s.data: {:p}", &s.data);
        println!("s.ptr : {:p}\n", s.ptr);

        s
    }

    fn new_box(data: String) -> Box<Self> {
        let mut s = Box::new(Self {
            data,
            ptr: std::ptr::null(),
        });

        s.ptr = &s.data;

        println!("&s    : {:p}", &s);
        println!("s     : {:p}", s);
        println!("s.data: {:p}", &s.data);
        println!("s.ptr : {:p}\n", s.ptr);

        s
    }
}

pub fn test() {
    let a = SelfRef::new("hello".into());
    println!("a     : {:p}", &a);
    println!("a.data: {:p}", &a.data);
    println!("a.ptr : {:p}\n", a.ptr);

    let b = a;
    println!("b     : {:p}", &b);
    println!("b.data: {:p}", &b.data);
    println!("b.ptr : {:p}\n", b.ptr);

    let mut a = SelfRef {
        data: "hello".into(),
        ptr: std::ptr::null(),
    };
    a.ptr = &a.data;
    println!("a     : {:p}", &a);
    println!("a.data: {:p}", &a.data);
    println!("a.ptr : {:p}", a.ptr);
    println!("{}\n", unsafe { &(*a.ptr) });

    let b = a;
    println!("b     : {:p}", &b);
    println!("b.data: {:p}", &b.data);
    println!("b.ptr : {:p}", b.ptr);
    println!("{}\n", unsafe { &(*b.ptr) });

    let mut a = Box::new(SelfRef {
        data: "hello".into(),
        ptr: std::ptr::null(),
    });
    a.ptr = &a.data;
    println!("a     : {:p}", &a);
    println!("a     : {:p}", a);
    println!("a.data: {:p}", &a.data);
    println!("a.ptr : {:p}", a.ptr);
    println!("{}\n", unsafe { &(*a.ptr) });

    // error[E0382]: use of moved value: `*a`
    // let b = a;

    let b = *a;
    println!("b     : {:p}", &b);
    // error[E0277]: the trait bound `SelfRef: Pointer` is not satisfied
    // println!("b     : {:p}", b);
    println!("b.data: {:p}", &b.data);
    println!("b.ptr : {:p}", b.ptr);
    println!("{}\n", unsafe { &(*b.ptr) });

    // error[E0382]: use of moved value: `a`
    // let b = a;

    let mut a = Box::pin(SelfRef {
        data: "hello".into(),
        ptr: std::ptr::null(),
    });
    a.ptr = &a.data;
    println!("a     : {:p}", &a);
    println!("a     : {:p}", a);
    println!("a.data: {:p}", &a.data);
    println!("a.ptr : {:p}", a.ptr);
    println!("{}\n", unsafe { &(*a.ptr) });

    // error[E0507]: cannot move out of dereference of `Pin<Box<SelfRef>>`
    // let b = *a;

    let b = a;
    // error[E0382]: borrow of moved value: `a`
    // println!("{}", a.data);

    let a = SelfRef::new_box("hello".into());
    println!("&a    : {:p}", &a);
    println!("a     : {:p}", a);
    println!("a.data: {:p}", &a.data);
    println!("a.ptr : {:p}", a.ptr);
    println!("{}\n", unsafe { &(*a.ptr) });
}
