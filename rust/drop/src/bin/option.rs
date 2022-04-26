use drop::Object;

fn main() {
    let tests: Vec<fn()> = vec![
        test1,
        test2,
        test3,
        test4,
    ];

    for test in tests {
        test();
        println!("");
    }
}

fn test1() {
    println!("/*** test 1 ***/");
    let mut objects = vec![Object{id: 1}, Object{id: 2}];
    let option = Some(Object{id: 3});

    if let Some(object) = option {
        objects.push(object);
    }
    println!("after if let");

    //println!("{:?}", option);
}

fn test2() {
    println!("/*** test 2 ***/");
    let _objects = vec![Object{id: 1}, Object{id: 2}];
    let option = Some(Object{id: 3});

    if let Some(_object) = option {
    }
    println!("after if let");

    //println!("{:?}", option);
}

fn test3() {
    println!("/*** test 3 ***/");
    let _objects = vec![Object{id: 1}, Object{id: 2}];
    let option = Some(Object{id: 3});

    if let Some(ref _object) = option {
    }
    println!("after if let");

    println!("{:?}", option);
}

fn test4() {
    println!("/*** test 4 ***/");
    let _objects = vec![Object{id: 1}, Object{id: 2}];
    let option = Some((Object{id: 3}, Object{id: 4}, Object{id: 5}));

    if let Some((_object3, ref _object4, _)) = option {
    }
    println!("after if let");

    //println!("{:?}", option);
}
