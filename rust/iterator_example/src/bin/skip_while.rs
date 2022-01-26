fn main() {
    //let a = vec![-1, 0, 1];
    let a = vec![-1i32, 0, 1];
    //let mut i = a.iter().skip_while(|x| x < 0);
    let mut i = a.iter().skip_while(|x| x.is_negative());
    //let mut i = a.iter().skip_while(|x| **x < 0);
    assert_eq!(Some(&0), i.next());
    assert_eq!(Some(&1), i.next());
    assert_eq!(None, i.next());
    println!("{:?}", a);
}
