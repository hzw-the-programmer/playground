Vec的Drop实现体现了结构体drop先调用，成员drop后调用。

Vec定义：
alloc\src\vec\mod.rs
```rust
pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
```

Vec的Drop实现：
```rust
unsafe impl<#[may_dangle] T, A: Allocator> Drop for Vec<T, A> {
    fn drop(&mut self) {
        unsafe {
            // use drop for [T]
            // use a raw slice to refer to the elements of the vector as weakest necessary type;
            // could avoid questions of validity in certain cases
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
        }
        // RawVec handles deallocation
    }
}
```
只负责drop元素，释放内存交给成员RawVec。

RawVec的定义：
alloc\src\raw_vec.rs
```rust
pub(crate) struct RawVec<T, A: Allocator = Global> {
    ptr: Unique<T>,
    /// Never used for ZSTs; it's `capacity()`'s responsibility to return usize::MAX in that case.
    ///
    /// # Safety
    ///
    /// `cap` must be in the `0..=isize::MAX` range.
    cap: Cap,
    alloc: A,
}
```

RawVec的Drop实现：
```rust
unsafe impl<#[may_dangle] T, A: Allocator> Drop for RawVec<T, A> {
    /// Frees the memory owned by the `RawVec` *without* trying to drop its contents.
    fn drop(&mut self) {
        if let Some((ptr, layout)) = self.current_memory() {
            unsafe { self.alloc.deallocate(ptr, layout) }
        }
    }
}
```

