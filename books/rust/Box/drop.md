Box成员T的drop会由编译器先调用，再调用Box的drop。
T的deinit由T的drop负责，Box的drop释放内存。

这和普通结构体drop顺序相反。
由于结构体在drop时，可以访问其成员，所以成员不能先drop。
参看例子：examples\drop\src\test3.rs
参看Vec：alloc\src\vec\mod.rs

Box的定义及Drop实现：
alloc\src\boxed.rs
```rust
pub struct Box<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
>(Unique<T>, A);

unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for Box<T, A> {
    fn drop(&mut self) {
        // the T in the Box is dropped by the compiler before the destructor is run
        let ptr = self.0;
        unsafe {
            let layout = Layout::for_value_raw(ptr.as_ptr());
            if layout.size() != 0 {
                self.1.deallocate(From::from(ptr.cast()), layout);
            }
        }
    }
}
```

创建Box时，先分配内存，再写入T。
```rust
impl<T, A: Allocator> Box<T, A> {
    pub fn new_in(x: T, alloc: A) -> Self
    where
        A: Allocator,
    {
        let mut boxed = Self::new_uninit_in(alloc);
        unsafe {
            boxed.as_mut_ptr().write(x);
            boxed.assume_init()
        }
    }
    
    pub fn new_uninit_in(alloc: A) -> Box<mem::MaybeUninit<T>, A>
    where
        A: Allocator,
    {
        let layout = Layout::new::<mem::MaybeUninit<T>>();
        // NOTE: Prefer match over unwrap_or_else since closure sometimes not inlineable.
        // That would make code size bigger.
        match Box::try_new_uninit_in(alloc) {
            Ok(m) => m,
            Err(_) => handle_alloc_error(layout),
        }
    }

    pub fn try_new_uninit_in(alloc: A) -> Result<Box<mem::MaybeUninit<T>, A>, AllocError>
    where
        A: Allocator,
    {
        let ptr = if T::IS_ZST {
            NonNull::dangling()
        } else {
            let layout = Layout::new::<mem::MaybeUninit<T>>();
            alloc.allocate(layout)?.cast()
        };
        unsafe { Ok(Box::from_raw_in(ptr.as_ptr(), alloc)) }
    }

    pub const unsafe fn from_raw_in(raw: *mut T, alloc: A) -> Self {
        Box(unsafe { Unique::new_unchecked(raw) }, alloc)
    }
}
```
