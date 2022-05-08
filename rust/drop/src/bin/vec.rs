use drop::Object;
use std::cmp::Reverse;

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2, test3, test4, test5, test6,
        test7, // test8, test9, test10, test11,
              // test12, test13, test14, test15, test16, test17, test18, test19, test20, test21, test22,
              // test23, test24,
    ];

    drop::tests(&tests);
}

fn test0() {
    let mut v = vec![];

    {
        let o0 = Object { id: 0 };
        let o1 = Object { id: 1 };
        let o2 = Object { id: 2 };
        v.push(o0);
        v.push(o1);
    }

    {
        let o3 = Object { id: 3 };
        let o4 = Object { id: 4 };
        v[0] = o3;
    }

    println!("finish");
}

fn test1() {
    let mut v1 = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
        Object { id: 4 },
    ];

    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );

    let v2 = v1.split_off(2);

    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    println!(
        "v2.len() == {}, v2.capacity() = {}",
        v2.len(),
        v2.capacity(),
    );
}

fn test2() {
    let mut v1 = vec![];
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    v1.push(Object { id: 0 });
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    v1.push(Object { id: 1 });
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    v1.push(Object { id: 2 });
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    v1.push(Object { id: 3 });
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    v1.push(Object { id: 4 });
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
}

fn test3() {
    let mut v1 = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
        Object { id: 4 },
    ];
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );

    {
        let d = v1.drain(2..);
    }

    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
}

fn test4() {
    let mut v1 = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
        Object { id: 4 },
    ];

    let d = v1.drain(2..);
    let v2: Vec<Object> = d.collect();
    println!(
        "v2.len() == {}, v2.capacity() = {}",
        v2.len(),
        v2.capacity(),
    );

    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
}

fn test5() {
    let mut v1 = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
        Object { id: 4 },
    ];

    v1.clear();
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
}

fn test6() {
    let mut v1 = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
        Object { id: 4 },
    ];
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    //let e = &v1[0];
    let i = v1.iter();
    v1.push(Object { id: 5 });
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    // println!("{:?}", e);
    // for e in i {
    //     println!("{:?}", e);
    // }
}

fn test7() {
    let mut v1 = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
        Object { id: 4 },
    ];
    v1.sort_by_key(|eb| Reverse(eb.id));
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
}

#[cfg(test)]
mod tests {
    use drop::Object;

    #[test]
    fn iter() {
        let mut v = vec![
            Object { id: 0 },
            Object { id: 1 },
            Object { id: 2 },
            Object { id: 3 },
            Object { id: 4 },
        ];
        let mut i = v.iter();
        println!("eq 0");
        assert_eq!(Some(&Object { id: 0 }), i.next());
        println!("eq 1");
        assert_eq!(Some(&Object { id: 1 }), i.next());
        println!("finish");
    }
}
