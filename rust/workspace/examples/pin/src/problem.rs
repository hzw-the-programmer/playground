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
}
