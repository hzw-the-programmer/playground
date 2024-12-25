RefCell实现了Clone，Ref没有实现Clone，但有一个clone关联函数，因为Ref实现了Deref，可以调用&T的clone。

如果T实现了Clone，那么RefCell也应该实现Clone。
```rust
impl<T: Clone> Clone for RefCell<T> {
    fn clone(&self) -> RefCell<T> {
        RefCell::new(self.borrow().clone())
    }
    fn clone_from(&mut self, other: &Self) {
        self.get_mut().clone_from(&other.borrow())
    }
}
```
这里self.borrow().clone()不是调用的Ref的clone方法，而是调用的T的clone方法，因为：
1. Ref实现了Deref，可以调用T的方法：
```rust
impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}
```
2. Ref的clone是关联函数而不是方法，需要Ref::clone()这个语法调用：
```rust
impl<'b, T: ?Sized> Ref<'b, T> {
    pub fn clone(orig: &Ref<'b, T>) -> Ref<'b, T> {
        Ref { value: orig.value, borrow: orig.borrow.clone() }
    }
}
```

BorrowRef实现了Clone
```rust
impl Clone for BorrowRef<'_> {
    fn clone(&self) -> Self {
        let borrow = self.borrow.get();
        self.borrow.set(borrow + 1);
        BorrowRef { borrow: self.borrow }
    }
}
```
