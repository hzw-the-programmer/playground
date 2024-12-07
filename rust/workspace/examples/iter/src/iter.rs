pub fn test() {
    // map();
    // for_each();
    // for_each_2();
    // for_each_3();
    // filter();
    // filter_2();
    // filter_3();
    filter_4();
}

fn map() {
    let a = [1, 2, 3];

    let mut i = a.iter();
    assert_eq!(Some(&1), i.next());
    assert_eq!(Some(&2), i.next());
    assert_eq!(Some(&3), i.next());
    assert_eq!(None, i.next());

    let mut i = a.iter().map(|x| {
        // let i: i32 = x;
        x * 2
    });
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(4), i.next());
    assert_eq!(Some(6), i.next());
    assert_eq!(None, i.next());

    assert_eq!([1, 2, 3], a);
}

fn for_each() {
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    // (0..5).map(|x| x * 2 + 1).for_each(|x| tx.send(x).unwrap());
    (0..5)
        .map(|x| x * 2 + 1)
        .for_each(move |x| tx.send(x).unwrap());
    let v: Vec<_> = rx.iter().collect();
    assert_eq!(vec![1, 3, 5, 7, 9], v);
}

fn for_each_2() {
    (0..5)
        .flat_map(|x| x * 100..x * 110)
        .for_each(|x| println!("{}", x));
}

fn for_each_3() {
    (0..5)
        .flat_map(|x| x * 100..x * 110)
        .enumerate()
        // .for_each(|x| println!("{:?}", x));
        .for_each(|(i, x)| println!("{i}:{x}"));
}

fn filter() {
    let a = [0i8, 1, 2];
    let mut i = a.iter().filter(|x| x.is_positive());
    assert_eq!(Some(&1), i.next());
    assert_eq!(Some(&2), i.next());
    assert_eq!(None, i.next());
}

fn filter_2() {
    let a = [0, 1, 2];
    let mut i = a.iter().filter(|x| **x > 1);
    assert_eq!(Some(&2), i.next());
    assert_eq!(None, i.next());
}

fn filter_3() {
    let a = [0, 1, 2];
    let mut i = a.iter().filter(|&x| *x > 1);
    assert_eq!(Some(&2), i.next());
    assert_eq!(None, i.next());
}

fn filter_4() {
    let a = [0, 1, 2];
    let mut i = a.iter().filter(|&&x| x > 1);
    assert_eq!(Some(&2), i.next());
    assert_eq!(None, i.next());
}
