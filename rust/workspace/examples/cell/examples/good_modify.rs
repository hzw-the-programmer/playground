use std::cell::UnsafeCell;

fn good_modify(x: &UnsafeCell<i32>) {
    unsafe {
        let ptr = x.get(); // ✅ 编译器认可
        *ptr = 100;
    }
}

fn main() {
    let x = UnsafeCell::new(5);
    good_modify(&x);
}
