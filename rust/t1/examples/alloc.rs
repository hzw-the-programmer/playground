use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use t1::Foo;

fn main() {
    test1();
    test2();
}

fn test1() {
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
    }
}
