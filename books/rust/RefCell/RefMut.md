RefMut不能clone，因为对同一块内存，只能有一个可变引用。
但这段内存的多个不重复部分可以有多个可变引用。

RefMut结构体定义：
```rust
pub struct RefMut<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    borrow: BorrowRefMut<'b>,
}
```
NonNull指向T，BorrowRefMut指向引用计数
```rust
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}
struct BorrowRefMut<'b> {
    borrow: &'b Cell<BorrowFlag>,
}
```

RefMut的创建：
```rust
impl<T: ?Sized> RefCell<T> {
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        match BorrowRefMut::new(&self.borrow) {
            Some(b) => {
                let value = unsafe { NonNull::new_unchecked(self.value.get()) };
                Ok(RefMut { value, borrow: b, marker: PhantomData })
            }
            None => Err(BorrowMutError {
                location: self.borrowed_at.get().unwrap(),
            }),
        }
    }
}
```

RefMut::new负责检查引用计数并将计数-1
```rust
impl<'b> BorrowRefMut<'b> {
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRefMut<'b>> {
        match borrow.get() {
            UNUSED => {
                borrow.set(UNUSED - 1);
                Some(BorrowRefMut { borrow })
            }
            _ => None,
        }
    }
}
```

RefMut::drop恢复引用计数
```rust
impl Drop for BorrowRefMut<'_> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow + 1);
    }
}
```

try_borrow_mut创建的RefMut是对整个T的可变引用，只能存在一份，所以不支持clone。
没有实现Clone，也没有clone方法或clone关联函数。

可把对T的整个可变引用分解为对T的不重复部分的可变引用，类似：
```rust
impl<'b, T: ?Sized> RefMut<'b, T> {
    pub fn map_split<U: ?Sized, V: ?Sized, F>(
        mut orig: RefMut<'b, T>,
        f: F,
    ) -> (RefMut<'b, U>, RefMut<'b, V>)
    where
        F: FnOnce(&mut T) -> (&mut U, &mut V),
    {
        let borrow = orig.borrow.clone();
        let (a, b) = f(&mut *orig);
        (
            RefMut { value: NonNull::from(a), borrow, marker: PhantomData },
            RefMut { value: NonNull::from(b), borrow: orig.borrow, marker: PhantomData },
        )
    }
}
```
所以BorrowRefMut定义了一个clone方法，注意不是实现Clone。
```rust
impl<'b> BorrowRefMut<'b> {
    fn clone(&self) -> BorrowRefMut<'b> {
        let borrow = self.borrow.get();
        self.borrow.set(borrow - 1);
        BorrowRefMut { borrow: self.borrow }
    }
}
```
