pub fn test() {
    // get();
    // get_mut();
    // get_unchecked();
    get_unchecked_mut();
}

// src\slice\mod.rs
fn get() {
    let v = [1, 2, 3];
    assert_eq!(v.get(1), Some(&2));
    assert_eq!(v.get(0..2), Some(&[1, 2][..]));
    assert_eq!(v.get(3), None);
    assert_eq!(v.get(0..4), None);
}

fn get_mut() {
    let mut v = [1, 2, 3];
    if let Some(e) = v.get_mut(1) {
        *e = 4;
    }
    assert_eq!(v, [1, 4, 3]);
}

fn get_unchecked() {
    let v = [1, 2, 3];
    unsafe {
        assert_eq!(v.get_unchecked(1), &2);
    }
}

fn get_unchecked_mut() {
    let mut v = [1, 2, 3];
    unsafe {
        let e = v.get_unchecked_mut(1);
        *e = 4;
    }
    assert_eq!(v, [1, 4, 3]);
}
