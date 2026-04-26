struct SelfRef {
    data: i32,
    ptr: *const i32,
}

impl SelfRef {
    fn new(data: i32) -> Self {
        let mut a = Self {
            data,
            ptr: std::ptr::null(),
        };

        a.ptr = &a.data;

        a.show();

        a
    }

    fn show(&self) {
        println!("&self.data: {:p}", &self.data);
        println!("&self.ptr : {:p}", &self.ptr);
        println!("self.ptr  : {:p}\n", self.ptr);
    }

    fn new_box(data: i32) -> Box<Self> {
        let mut a = Box::new(Self {
            data,
            ptr: std::ptr::null(),
        });

        a.ptr = &a.data;

        a.show();

        a
    }
}

pub fn test() {
    let a = SelfRef::new(1);
    a.show();

    let a = SelfRef::new_box(1);
    a.show();

    let b = *a;
    b.show();
}
