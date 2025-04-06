use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

struct MyGlobalAlloc;

unsafe impl GlobalAlloc for MyGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // #[warn(unsafe_op_in_unsafe_fn)]
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            COUNTER.fetch_add(layout.size(), Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // #[warn(unsafe_op_in_unsafe_fn)]
        unsafe {
            System.dealloc(ptr, layout);
        }
        COUNTER.fetch_sub(layout.size(), Relaxed);
    }
}

#[global_allocator]
static MY_GLOBAL_ALLOC: MyGlobalAlloc = MyGlobalAlloc;

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);
