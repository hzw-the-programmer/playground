## Rc的strong计数为0时，会drop T。weak计数为0时，才会deallocate内存。
```Rust
fn drop(&mut self) {
    unsafe {
        self.inner().dec_strong();
        if self.inner().strong() == 0 {
            // destroy the contained object
            ptr::drop_in_place(Self::get_mut_unchecked(self));

            // remove the implicit "strong weak" pointer now that we've
            // destroyed the contents.
            self.inner().dec_weak();

            if self.inner().weak() == 0 {
                self.alloc.deallocate(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
            }
        }
    }
}
```

## Rc是一个结构体，有一个ptr字段，指向堆上的RcBox
```Rust
pub struct Rc<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
> {
    ptr: NonNull<RcBox<T>>,
    phantom: PhantomData<RcBox<T>>,
    alloc: A,
}

pub fn new(value: T) -> Rc<T> {
    // There is an implicit weak pointer owned by all the strong
    // pointers, which ensures that the weak destructor never frees
    // the allocation while the strong destructor is running, even
    // if the weak pointer is stored inside the strong one.
    unsafe {
        Self::from_inner(
            Box::leak(Box::new(RcBox { strong: Cell::new(1), weak: Cell::new(1), value }))
                .into(),
        )
    }
}
```

## Rc不是Send的
```Rust
impl<T: ?Sized, A: Allocator> !Send for Rc<T, A> {}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\alloc\src\rc.rs
