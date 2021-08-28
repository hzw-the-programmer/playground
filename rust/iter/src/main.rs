fn main() {
    let mut v = vec!["hello", "world"];
    let mv1 = v.iter_mut().map(|x| x).collect::<Vec<_>>();
    let v1 = v.iter().map(|x| x).collect::<Vec<_>>();
    //let e1 = &mut v[0];
    //let e1 = v[0];
    println!("{:?}", v1);
    println!("{}", mv1);
}
