fn main() {
    let v = vec![1, 2, 3];
    let mut i = v.iter();
    assert_eq!(Some(&3), i.last());
    //assert_eq!(None, i.next());
    println!("{:?}", v);
}
