pub fn test() {
    // map();
    for_each();
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
