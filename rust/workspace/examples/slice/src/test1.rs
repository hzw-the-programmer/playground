pub fn test() {
    // get();
    get_mut();
}

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
