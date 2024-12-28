在不可变上下文中，container[i] 是 *container.index(i) 的语法糖。
let v = container[i];        // 1
let v = *container.index(i); // 2
式1、2是等价的。该例子要求T实现Copy。

在可变上下文中，container[i] 是 *container.index_mut(i) 的语法糖。
container[i] = v;            // 1
*container.index_mut(i) = v; // 2
式1、2是等价的。

这里的index、index_mut分别是Index、IndexMut特型的方法。
core\src\ops\index.rs
```rust
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}
pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

slice实现了Index、IndexMut。
core\src\slice\index.rs
```rust
impl<T, I> ops::Index<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}
impl<T, I> ops::IndexMut<I> for [T]
where
    I: SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        index.index_mut(self)
    }
}
```

SliceIndex trait
```rust
pub unsafe trait SliceIndex<T: ?Sized>: private_slice_index::Sealed {
    type Output: ?Sized;
    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: *const T) -> *const Self::Output;
    unsafe fn get_unchecked_mut(self, slice: *mut T) -> *mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}
```

usize实现了SliceIndex。
```rust
unsafe impl<T> SliceIndex<[T]> for usize {
    type Output = T;
    fn get(self, slice: &[T]) -> Option<&T> {
        if self < slice.len() { unsafe { Some(&*self.get_unchecked(slice)) } } else { None }
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut T> {
        if self < slice.len() { unsafe { Some(&mut *self.get_unchecked_mut(slice)) } } else { None }
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const T {
        unsafe {
            crate::hint::assert_unchecked(self < slice.len());
            slice.as_ptr().add(self)
        }
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut T {
        unsafe { slice.as_mut_ptr().add(self) }
    }
    fn index(self, slice: &[T]) -> &T {
        &(*slice)[self]
    }
    fn index_mut(self, slice: &mut [T]) -> &mut T {
        &mut (*slice)[self]
    }
}
```

Range<usize>实现SliceIndex
```rust
unsafe impl<T> SliceIndex<[T]> for ops::Range<usize> {
    type Output = [T];
    fn get(self, slice: &[T]) -> Option<&[T]> {
        if self.start > self.end || self.end > slice.len() {
            None
        } else {
            unsafe { Some(&*self.get_unchecked(slice)) }
        }
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        if self.start > self.end || self.end > slice.len() {
            None
        } else {
            unsafe { Some(&mut *self.get_unchecked_mut(slice)) }
        }
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe {
            let new_len = unchecked_sub(self.end, self.start);
            ptr::slice_from_raw_parts(slice.as_ptr().add(self.start), new_len)
        }
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe {
            let new_len = unchecked_sub(self.end, self.start);
            ptr::slice_from_raw_parts_mut(slice.as_mut_ptr().add(self.start), new_len)
        }
    }
    fn index(self, slice: &[T]) -> &[T] {
        if self.start > self.end {
            slice_index_order_fail(self.start, self.end);
        } else if self.end > slice.len() {
            slice_end_index_len_fail(self.end, slice.len());
        }
        unsafe { &*self.get_unchecked(slice) }
    }
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        if self.start > self.end {
            slice_index_order_fail(self.start, self.end);
        } else if self.end > slice.len() {
            slice_end_index_len_fail(self.end, slice.len());
        }
        unsafe { &mut *self.get_unchecked_mut(slice) }
    }
}
```

RangeTo<usize>实现SliceIndex
```rust
unsafe impl<T> SliceIndex<[T]> for ops::RangeTo<usize> {
    type Output = [T];
    fn get(self, slice: &[T]) -> Option<&[T]> {
        (0..self.end).get(slice)
    }

    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        (0..self.end).get_mut(slice)
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe { (0..self.end).get_unchecked(slice) }
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe { (0..self.end).get_unchecked_mut(slice) }
    }
    fn index(self, slice: &[T]) -> &[T] {
        (0..self.end).index(slice)
    }
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        (0..self.end).index_mut(slice)
    }
}
```

RangeFrom<usize>实现SliceIndex
```rust
unsafe impl<T> SliceIndex<[T]> for ops::RangeFrom<usize> {
    type Output = [T];
    fn get(self, slice: &[T]) -> Option<&[T]> {
        (self.start..slice.len()).get(slice)
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        (self.start..slice.len()).get_mut(slice)
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe { (self.start..slice.len()).get_unchecked(slice) }
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe { (self.start..slice.len()).get_unchecked_mut(slice) }
    }
    fn index(self, slice: &[T]) -> &[T] {
        if self.start > slice.len() {
            slice_start_index_len_fail(self.start, slice.len());
        }
        unsafe { &*self.get_unchecked(slice) }
    }
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        if self.start > slice.len() {
            slice_start_index_len_fail(self.start, slice.len());
        }
        unsafe { &mut *self.get_unchecked_mut(slice) }
    }
}
```

RangeFull实现SliceIndex
```rust
unsafe impl<T> SliceIndex<[T]> for ops::RangeFull {
    type Output = [T];
    fn get(self, slice: &[T]) -> Option<&[T]> {
        Some(slice)
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        Some(slice)
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        slice
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        slice
    }
    fn index(self, slice: &[T]) -> &[T] {
        slice
    }
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        slice
    }
}
```

RangeInclusive<usize>实现SliceIndex
```rust
unsafe impl<T> SliceIndex<[T]> for ops::RangeInclusive<usize> {
    type Output = [T];
    fn get(self, slice: &[T]) -> Option<&[T]> {
        if *self.end() == usize::MAX { None } else { self.into_slice_range().get(slice) }
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        if *self.end() == usize::MAX { None } else { self.into_slice_range().get_mut(slice) }
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe { self.into_slice_range().get_unchecked(slice) }
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe { self.into_slice_range().get_unchecked_mut(slice) }
    }
    fn index(self, slice: &[T]) -> &[T] {
        if *self.end() == usize::MAX {
            slice_end_index_overflow_fail();
        }
        self.into_slice_range().index(slice)
    }
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        if *self.end() == usize::MAX {
            slice_end_index_overflow_fail();
        }
        self.into_slice_range().index_mut(slice)
    }
}
```

RangeToInclusize<usize>实现SliceIndex
```rust
unsafe impl<T> SliceIndex<[T]> for ops::RangeToInclusive<usize> {
    type Output = [T];
    fn get(self, slice: &[T]) -> Option<&[T]> {
        (0..=self.end).get(slice)
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        (0..=self.end).get_mut(slice)
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe { (0..=self.end).get_unchecked(slice) }
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe { (0..=self.end).get_unchecked_mut(slice) }
    }
    fn index(self, slice: &[T]) -> &[T] {
        (0..=self.end).index(slice)
    }
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        (0..=self.end).index_mut(slice)
    }
}
```

IndexRange实现SliceIndex
```rust
unsafe impl<T> SliceIndex<[T]> for ops::IndexRange {
    type Output = [T];
    fn get(self, slice: &[T]) -> Option<&[T]> {
        if self.end() <= slice.len() {
            unsafe { Some(&*self.get_unchecked(slice)) }
        } else {
            None
        }
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        if self.end() <= slice.len() {
            unsafe { Some(&mut *self.get_unchecked_mut(slice)) }
        } else {
            None
        }
    }
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe { ptr::slice_from_raw_parts(slice.as_ptr().add(self.start()), self.len()) }
    }
    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut [T] {
        unsafe { ptr::slice_from_raw_parts_mut(slice.as_mut_ptr().add(self.start()), self.len()) }
    }
    fn index(self, slice: &[T]) -> &[T] {
        if self.end() <= slice.len() {
            unsafe { &*self.get_unchecked(slice) }
        } else {
            slice_end_index_len_fail(self.end(), slice.len())
        }
    }
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        if self.end() <= slice.len() {
            unsafe { &mut *self.get_unchecked_mut(slice) }
        } else {
            slice_end_index_len_fail(self.end(), slice.len())
        }
    }
}
```
