fn main() {
    let a = vec![0, 1, 2, 3, 4, 5, 6];
    let mut i = a.iter().step_by(2);
    assert_eq!(Some(&0), i.next());
    assert_eq!(Some(&2), i.next());
    assert_eq!(Some(&4), i.next());
    assert_eq!(Some(&6), i.next());
    assert_eq!(None, i.next());
    println!("{:?}", a);
}
