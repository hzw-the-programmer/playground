RefCell结构体提供了T的存储空间及引用计数。
```rust
pub struct RefCell<T: ?Sized> {
    value: UnsafeCell<T>,
    borrow: Cell<BorrowFlag>,
}
```

Ref结构体通过NonNull<T>指针引用RefCell::value。
通过BorrowRef管理引用计数。BorrowRef不是pub的，没有对外开放。
```rust
pub struct Ref<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    borrow: BorrowRef<'b>,
}
struct BorrowRef<'b> {
    borrow: &'b Cell<BorrowFlag>,
}
```

BorrowRef::new，引用计数+1；Borrowref::drop引用计数减一
```rust
impl<'b> BorrowRef<'b> {
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRef<'b>> {
        let b = borrow.get().wrapping_add(1);
        if !is_reading(b) {
            None
        } else {
            borrow.set(b);
            Some(BorrowRef { borrow })
        }
    }
}
impl Drop for BorrowRef<'_> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow - 1);
    }
}
```

从RefCell借用T。
```rust
pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
    match BorrowRef::new(&self.borrow) {
        Some(b) => {
            let value = unsafe { NonNull::new_unchecked(self.value.get()) };
            Ok(Ref { value, borrow: b })
        }
        None => Err(BorrowError {
            location: self.borrowed_at.get().unwrap(),
        }),
    }
}
```

如果Ref直接引用RefCell::borrow，像这样：
```rust
pub struct Ref<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    borrow: &'b Cell<BorrowFlag>,
}
```
那么Ref就必须实现Drop。那么分解Ref成员则不可能。即类似这样的调用不可能：
```rust
pub fn map<U: ?Sized, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
where
    F: FnOnce(&T) -> &U,
{
    Ref { value: NonNull::from(f(&*orig)), borrow: orig.borrow }
}
```
这里分解了Ref，只用了Ref的borrow字段。
参见这个例子：examples\drop\src\test2.rs
