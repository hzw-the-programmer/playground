fn main() {
    let v = vec![1, 2, 3];
    let mut i = v.iter();
    assert_eq!(Some(&1), i.nth(0));
    assert_eq!(Some(&2), i.nth(0));
    assert_eq!(Some(&3), i.nth(0));
    assert_eq!(None, i.nth(0));
    println!("{:?}", v);
}
