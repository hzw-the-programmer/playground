use core::marker::PhantomPinned;
use core::pin::Pin;

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

    fn new_box_pin(data: i32) -> Pin<Box<Self>> {
        let mut a = Box::pin(Self {
            data,
            ptr: std::ptr::null(),
        });

        a.ptr = &a.data;

        a.show();

        a
    }
}

struct SelfRefPined {
    data: i32,
    ptr: *const i32,
    _pined: PhantomPinned,
}

impl SelfRefPined {
    fn new_1() -> Self {
        let data = 1;
        let ptr = &data as *const i32;
        println!("{:p}", &data);
        println!("{:p}", ptr);
        let a = Self {
            data,
            ptr,
            _pined: PhantomPinned,
        };
        println!("{:p}", &a.data);
        a
    }

    fn new_2() -> Box<Self> {
        let a = Self {
            data: 1,
            ptr: core::ptr::null(),
            _pined: PhantomPinned,
        };
        println!("{:p}", &a.data);
        let a = Box::new(a);
        println!("{:p}", &a.data);
        a
    }
}

pub fn test() {
    assert_unpin(SelfRef {
        data: 1,
        ptr: std::ptr::null(),
    });
    // assert_unpin(SelfRefPined {
    //     data: 1,
    //     ptr: std::ptr::null(),
    //     _pined: PhantomPinned,
    // });

    let a = SelfRef::new(1);
    a.show();

    let a = SelfRef::new_box(1);
    a.show();

    let b = *a;
    b.show();

    let a = SelfRef::new_box_pin(1);
    a.show();

    // let b = *a;
    // b.show();

    let a = SelfRefPined::new_1();
    println!("{:p}\n", &a.data);
    let a = SelfRefPined::new_2();
    println!("{:p}\n", &a.data);
}

fn assert_unpin<T: Unpin>(_t: T) -> T {
    _t
}
