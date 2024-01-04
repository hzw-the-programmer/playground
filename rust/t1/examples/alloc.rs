use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};

fn main() {
    test1();
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
