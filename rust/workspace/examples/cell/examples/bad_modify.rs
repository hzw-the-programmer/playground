#[allow(invalid_reference_casting)]

fn bad_modify(x: &i32) {
    unsafe {
        let ptr = x as *const i32 as *mut i32;
        // error: assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
        *ptr = 100; // ❌ 未定义行为！
    }
}

fn main() {
    let a = 5;
    bad_modify(&a);
}
