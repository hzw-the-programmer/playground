pub fn test() {
    test1();
    test2();
    test3();
}

fn test1() {
    let i = 32;
    println!("{i}");
    let r = &i;
    let ptr = r as *const i32 as *mut i32;

    {
        // assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
        // unsafe { *ptr = 33 };

        // unsafe { *get(r) = 33 };

        // unsafe { ptr.write(33) };
    }

    {
        // casting `&T` to `&mut T` is undefined behavior, even if the reference is unused, consider instead using an `UnsafeCell`
        // let r = unsafe { &mut *ptr };

        // let r = unsafe { &mut *get(r) };

        // let r = &mut unsafe { *ptr };
        let r = &mut unsafe { *get(r) };

        *r = 33;
    }

    println!("{i}");

    fn get(r: &i32) -> *mut i32 {
        r as *const i32 as *mut i32
    }
}

fn test2() {
    let i = UnsafeCellCustom { v: 32 };
    println!("{i:?}");
    let r = &i;
    let ptr = r as *const UnsafeCellCustom<i32> as *mut UnsafeCellCustom<i32>;

    {
        // assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
        // unsafe { *ptr = UnsafeCellCustom { v: 33 } };

        // unsafe { *get(r) = UnsafeCellCustom { v: 33 } };

        // unsafe { ptr.write(UnsafeCellCustom { v: 33 }) };
    }

    {
        // casting `&T` to `&mut T` is undefined behavior, even if the reference is unused, consider instead using an `UnsafeCell`
        // let r = unsafe { &mut *ptr };

        let r = unsafe { &mut *get(r) };

        // cannot move out of `*ptr` which is behind a raw pointer
        // let r = &mut unsafe { *ptr };
        // cannot move out of a raw pointer
        // let r = &mut unsafe { *get(r) };

        *r = UnsafeCellCustom { v: 33 };
    }

    println!("{i:?}");

    fn get<T>(r: &UnsafeCellCustom<T>) -> *mut UnsafeCellCustom<T> {
        r as *const UnsafeCellCustom<T> as *mut UnsafeCellCustom<T>
    }
}

fn test3() {
    let i = UnsafeCellCustom { v: 32 };
    println!("{i:?}");
    let r = &i;
    let ptr = r as *const UnsafeCellCustom<i32> as *const i32 as *mut i32;

    {
        // assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
        // unsafe { *ptr = 33 };

        // unsafe { *i.get() = 33 };

        // unsafe { ptr.write(33) };
    }

    {
        // casting `&T` to `&mut T` is undefined behavior, even if the reference is unused, consider instead using an `UnsafeCell`
        // let r = unsafe { &mut *ptr };

        // let r = unsafe { &mut *i.get() };

        // let r = &mut unsafe { *ptr };
        let r = &mut unsafe { *i.get() };

        *r = 33;
    }

    println!("{i:?}");

    i.replace(34);
    println!("{i:?}");
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
        // cannot move out of a raw pointer
        // std::mem::replace(&mut unsafe { *self.get() }, n)
    }
}
