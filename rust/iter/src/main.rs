fn main() {
    test1();
    test2();
}

fn test1() {
    let mut v = vec!["hello", "world"];
    let mv1 = v.iter_mut().map(|x| x).collect::<Vec<_>>();
    let v1 = v.iter().map(|x| x).collect::<Vec<_>>();
    //let e1 = &mut v[0];
    //let e1 = v[0];
    //println!("{:?}", v1);
    //println!("{}", mv1);
}

fn test2() {
    let players = vec!["Jane", "Jill", "Jack", "John"];
    let total = players
        .iter()
        .map(|x| x.len())
        .fold(0, |acc, next| acc + next);
    assert_eq!(total, 16);
    println!("{:?}", players);
}
