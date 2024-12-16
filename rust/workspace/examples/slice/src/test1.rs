pub fn test() {
    get();
}

fn get() {
    let v = [1, 2, 3];
    assert_eq!(v.get(1), Some(&2));
    assert_eq!(v.get(0..2), Some(&[1, 2][..]));
    assert_eq!(v.get(3), None);
    assert_eq!(v.get(0..4), None);
}
