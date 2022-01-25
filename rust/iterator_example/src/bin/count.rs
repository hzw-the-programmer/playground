fn main() {
    let v = vec![1, 2, 3];
    let i = v.iter();
    assert_eq!(3, i.count());
    //assert_eq!(None, i.next());
    println!("{:?}", v);
}
