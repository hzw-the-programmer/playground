fn main() {
    test1();
    test2();
    test3();
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

#[derive(Debug)]
struct Foo {
    i: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop {}", self.i);
    }
}

fn test3() {
    let mut v = Vec::new();

    for i in 0..5 {
        let f = Foo { i };
        v.push(f);
    }

    let mut i = v.iter();
    println!("{:?}", i.nth(1));
    println!("{:?}", i.nth(1));
    println!("{}", v.len());

    // for i in v {
    // for i in &v {
    for i in &mut v {
        // println!("b {}", v.len());
        println!("b");
        i.i += 1;
        println!("{:?}", i);
        println!("a");
    }

    println!("finish");
}
