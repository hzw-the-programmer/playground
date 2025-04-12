fn main() {
    test1();
}

fn test1() {
    // let i = 32;
    // let r = &i as *const i32 as *mut i32;
    // let i: i32 = r;
    // let r = unsafe { &mut *r };
    // *r = 33;
    // std::mem::replace(unsafe { &mut *r }, 33);
    // unsafe { std::ptr::write(r, 33); }
    // println!("{i}");

    {
        let i = 32;
        println!("{i}");
        let r = &i;
        let r = &mut unsafe { *(r as *const i32 as *mut i32) };
        // let r = unsafe { &mut *(r as *const i32 as *mut i32) };
        *r = 33;
        println!("{i}");
    }

    {
        let i = 32;
        println!("{i}");
        unsafe {
            let i = &i;
            let i = i as *const i32 as *mut i32;
            // *i = 33;
            // let _ = std::mem::replace(unsafe { &mut *i }, 33);
            i.write(33);
        }
        println!("{i}");
    }

    {
        let i = 32;
        println!("{i}");
        unsafe {
            (&i as *const i32 as *mut i32).write(33);
        }
        println!("{i}");
    }

    {
        let i = UnsafeCellCustom { v: 32 };
        println!("{i:?}");

        unsafe {
            let ptr = &i as *const UnsafeCellCustom<i32> as *const i32 as *mut i32;
            // *ptr = 33;
            // let _ = std::mem::replace(unsafe { &mut *ptr }, 33);
            // let _ = std::mem::replace(unsafe { &mut *i.get() }, 33);
            let _ = std::mem::replace(unsafe { &mut *get(&i) }, 33);
            // let _ = std::mem::replace(&mut unsafe { *ptr }, 33);
            // let _ = std::mem::replace(&mut unsafe { *i.get() }, 33);
            // ptr.write(33);
        }
        println!("{i:?}");
    }

    // {
    //     let i = UnsafeCellCustom { v: 32 };
    //     println!("{i:?}");

    //     unsafe {
    //         let i = &i as *const UnsafeCellCustom<i32> as *mut UnsafeCellCustom<i32>;
    //         // let i: i32 = i;
    //         // *i = UnsafeCellCustom { v: 33 };
    //         // let _ = std::mem::replace(unsafe { &mut *i }, UnsafeCellCustom { v: 33 });
    //         // unsafe {
    //         //     let _ = std::mem::replace(&mut *i, UnsafeCellCustom { v: 33 });
    //         // }
    //         // let _ = std::mem::replace(&mut unsafe { *i }, UnsafeCellCustom { v: 33 });
    //         i.write(UnsafeCellCustom { v: 33 });
    //     }
    //     println!("{i:?}");
    // }

    // {
    //     let i = UnsafeCellCustom { v: 32 };
    //     println!("{i:?}");

    //     unsafe {
    //         i.get().write(33);
    //     }
    //     println!("{i:?}");

    //     i.replace(34);
    //     println!("{i:?}");
    // }
}

#[derive(Debug)]
struct UnsafeCellCustom<T> {
    v: T,
}

impl<T> UnsafeCellCustom<T> {
    fn get(&self) -> *mut T {
        self as *const UnsafeCellCustom<T> as *const T as *mut T
    }

    fn replace(&self, n: T) -> T {
        std::mem::replace(unsafe { &mut *self.get() }, n)
    }
}

fn get<T>(r: &UnsafeCellCustom<T>) -> *mut T {
    r as *const UnsafeCellCustom<T> as *const T as *mut T
}
