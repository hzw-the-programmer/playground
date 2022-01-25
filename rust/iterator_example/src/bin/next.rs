fn main() {
    let a = vec![1, 2, 3];

    let mut i = a.iter();

    assert_eq!(Some(&1), i.next());
    assert_eq!(Some(&2), i.next());
    assert_eq!(Some(&3), i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
}
