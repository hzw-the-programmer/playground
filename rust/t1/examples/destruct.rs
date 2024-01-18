use t1::Foo;

fn test1() {
    println!("\ntest1\n");
    let v = vec![Foo { id: 1 }, Foo { id: 2 }];
    // let i = v.iter().filter(|&&e| {
    let i = v.iter().filter(|&e| {
        // let i = v.iter().filter(|e| {
        // let n: i32 = e;
        println!("in predicate {e:?}");
        e.id > 1
    });
    println!("before for");
    for e in i {
        println!("in for {e:?}");
    }
    println!("finish");
}

fn main() {
    test1();
}
