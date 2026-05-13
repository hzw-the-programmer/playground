use std::cell::UnsafeCell;

// 内部可变性容器
struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(v: T) -> Self {
        Self {
            value: UnsafeCell::new(v),
        }
    }

    // 重点：&self 不可变，但能修改内部！
    pub fn set(&self, v: T) {
        unsafe {
            *self.value.get() = v;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

// 使用
fn main() {
    let x = MyCell::new(10);

    x.set(20); // 注意：这里是 &x，不是 &mut x！
    println!("{}", x.get()); // 20
}
