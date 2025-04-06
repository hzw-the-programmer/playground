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
use std::ptr::{self, NonNull};

pub struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            NonNull::new_unchecked(ptr.cast::<T>())
        };
        unsafe {
            ptr::write(ptr.as_ptr(), value);
        }
        MyBox { ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // 步骤 1: 调用 T 的 Drop 方法（如果 T 实现了 Drop trait）
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
        }

        // 步骤 2: 释放堆上分配的内存
        let layout = Layout::new::<T>();
        unsafe {
            dealloc(self.ptr.as_ptr().cast::<u8>(), layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<Option<Box<i32>>>(), 8);
        assert_eq!(std::mem::size_of::<Option<MyBox<i32>>>(), 8);
    }
}

pub mod main {
    use super::*;
    use crate::my_global_alloc::COUNTER;
    use std::sync::atomic::Ordering::Relaxed;

    pub fn test1() {
        println!("before main: {}", COUNTER.load(Relaxed));

        println!("before block: {}", COUNTER.load(Relaxed));
        {
            let _mb = MyBox::new(Foo { id: 1 });
            println!("after new: {}", COUNTER.load(Relaxed));
        }
        println!("after block: {}", COUNTER.load(Relaxed));
    }

    struct Foo {
        id: i32,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Drop Foo {}", self.id);
        }
    }
}
