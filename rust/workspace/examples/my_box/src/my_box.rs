/*
std\src\alloc.rs
pub use alloc_crate::alloc::*;

alloc\src\alloc.rs
pub use core::alloc::*;
*/
use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
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

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // 步骤 1: 调用 T 的 Drop 方法（如果 T 实现了 Drop trait）
        unsafe {
            ptr::drop_in_place(self.ptr);
        }

        // 步骤 2: 释放堆上分配的内存
        let layout = Layout::new::<T>();
        unsafe {
            dealloc(self.ptr.cast::<u8>(), layout);
        }
    }
}
