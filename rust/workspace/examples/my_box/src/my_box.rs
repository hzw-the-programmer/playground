/*
std\src\alloc.rs
pub use alloc_crate::alloc::*;

alloc\src\alloc.rs
pub use core::alloc::*;
*/
use std::alloc::{Layout, alloc, handle_alloc_error};
/*
std\src\lib.rs
pub use core::ptr;
*/
use std::ptr;

pub struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            ptr.cast::<T>()
        };
        unsafe {
            ptr::write(ptr, value);
        }
        MyBox { ptr }
    }
}
