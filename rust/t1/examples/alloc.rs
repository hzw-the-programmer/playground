use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ptr;
use t1::Foo;

fn main() {
    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    println!("\ntest1\n");
    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        *(ptr as *mut u16) = 23;
        assert_eq!(23, *(ptr as *mut u16));
        dealloc(ptr, layout);
    }
}

fn test2() {
    println!("\ntest2\n");
    unsafe {
        let layout = Layout::new::<Foo>();
        let ptr = alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        let f = ptr as *const Foo;
        println!("{:?}", f);
        println!("{}", (*f).id);
        // let f = ptr as &Foo;
        // let f = f as &Foo;
        let f = &*f;
        println!("{:?}", f);
        dealloc(ptr, layout);
    }
}

fn test3() {
    println!("\ntest3\n");
    unsafe {
        let layout = Layout::new::<Foo>();
        let ptr = alloc(layout) as *mut Foo;
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        let f = Box::from_raw(ptr);
        // let f1 = Box::from_raw(ptr);
    }
}

fn test4() {
    println!("\ntest4\n");
    let f = Foo { id: 1 };
    let b = Box::new(f);
    let p = Box::into_raw(b);
    unsafe {
        ptr::drop_in_place(p);
        dealloc(p as *mut u8, Layout::new::<Foo>());
    }
}
