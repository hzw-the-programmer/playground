pub fn test() {
    // map();

    // for_each();
    // for_each_2();
    // for_each_3();

    // filter();
    // filter_2();
    // filter_3();
    // filter_4();

    // filter_map();
    // filter_map_2();

    // enumerate();

    // peekable();
    // peekable_2();

    // skip_while();
    // skip_while_2();
    // skip_while_3();

    // take_while();
    // take_while_2();
    // take_while_3();
    // take_while_4();
    // take_while_5();

    // map_while();
    // map_while_2();
    // map_while_3();
    map_while_4();
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

fn filter_map() {
    let a = ["1", "two", "NaN", "four", "5"];
    let mut i = a
        .iter()
        .map(|s| s.parse())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(5), i.next());
    assert_eq!(None, i.next());
}

fn filter_map_2() {
    let a = ["1", "two", "NaN", "four", "5"];
    let mut i = a.iter().filter_map(|s| s.parse().ok());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(5), i.next());
    assert_eq!(None, i.next());
}

fn enumerate() {
    let a = ['a', 'b', 'c'];
    let mut i = a.iter().enumerate();
    assert_eq!(Some((0, &'a')), i.next());
    assert_eq!(Some((1, &'b')), i.next());
    assert_eq!(Some((2, &'c')), i.next());
    assert_eq!(None, i.next());
}

fn peekable() {
    let a = [1, 2, 3];
    let mut i = a.iter().peekable();
    assert_eq!(Some(&&1), i.peek());
    assert_eq!(Some(&&1), i.peek());
    assert_eq!(Some(&1), i.next());
    assert_eq!(Some(&2), i.next());
    assert_eq!(Some(&3), i.next());
    assert_eq!(None, i.peek());
}

fn peekable_2() {
    let a = [1, 2, 3];
    let mut i = a.iter().peekable();
    assert_eq!(Some(&mut &1), i.peek_mut());
    if let Some(e) = i.peek_mut() {
        // let i :i32 = e;
        *e = &4;
    }
    assert_eq!(vec![&4, &2, &3], i.collect::<Vec<_>>());
    assert_eq!([1, 2, 3], a);
}

fn skip_while() {
    let a = [-1i32, 0, 1];
    // no method named `is_negative` found for reference `&&{integer}` in the current scope
    let mut i = a.iter().skip_while(|x| x.is_negative());
    assert_eq!(Some(&0), i.next());
    assert_eq!(Some(&1), i.next());
    assert_eq!(None, i.next());
}

fn skip_while_2() {
    let a = [-1, 0, 1];
    let mut i = a.iter().skip_while(|x| **x < 0);
    assert_eq!(Some(&0), i.next());
    assert_eq!(Some(&1), i.next());
    assert_eq!(None, i.next());
}

fn skip_while_3() {
    let a = [-1, 0, 1, -2];
    let mut i = a.iter().skip_while(|x| **x < 0);
    assert_eq!(Some(&0), i.next());
    assert_eq!(Some(&1), i.next());
    assert_eq!(Some(&-2), i.next());
    assert_eq!(None, i.next());
}

fn take_while() {
    let a = [-1i32, 0, 1];
    let mut i = a.iter().take_while(|x| x.is_negative());
    assert_eq!(Some(&-1), i.next());
    assert_eq!(None, i.next());
}

fn take_while_2() {
    let a = [-1, 0, 1];
    let mut i = a.iter().take_while(|x| **x < 0);
    assert_eq!(Some(&-1), i.next());
    assert_eq!(None, i.next());
}

fn take_while_3() {
    let a = [-1, 0, 1, -2];
    let mut i = a.iter().take_while(|x| **x < 0);
    assert_eq!(Some(&-1), i.next());
    assert_eq!(None, i.next());
}

fn take_while_4() {
    let a = [1, 2, 3, 4];
    let mut i = a.iter();
    let v: Vec<_> = i.by_ref().take_while(|x| **x != 3).collect();
    assert_eq!(vec![&1, &2], v);
    assert_eq!(vec![&4], i.collect::<Vec<_>>());
}

fn take_while_5() {
    let a = [1, 2, 3, 4];
    let mut i = a.iter();
    let v: Vec<_> = i.by_ref().take_while(|x| **x != 3).cloned().collect();
    assert_eq!(vec![1, 2], v);
    assert_eq!(vec![4], i.cloned().collect::<Vec<_>>());
}

fn map_while() {
    let a = [-1, 4, 0, 1];
    let mut i = a.iter().map_while(|x| 16i32.checked_div(*x));
    assert_eq!(Some(-16), i.next());
    assert_eq!(Some(4), i.next());
    assert_eq!(None, i.next());
}

fn map_while_2() {
    let a = [-1, 4, 0, 1];
    let mut i = a
        .iter()
        .map(|x| 16i32.checked_div(*x))
        .take_while(|x| x.is_some())
        .map(|x| x.unwrap());
    assert_eq!(Some(-16), i.next());
    assert_eq!(Some(4), i.next());
    assert_eq!(None, i.next());
}

fn map_while_3() {
    let a = [0, 1, 2, -3, 4, 5, -6];
    let v = a
        .iter()
        .map_while(|x| u32::try_from(*x).ok())
        .collect::<Vec<_>>();
    assert_eq!(vec![0, 1, 2], v);
    // can't compare `&[{integer}; 3]` with `Vec<u32>`
    // assert_eq!(&[0, 1, 2], v);
    assert_eq!(v, &[0, 1, 2]);
}

fn map_while_4() {
    let a = [1, 2, -3, 4];
    let mut i = a.iter();
    let v: Vec<_> = i.by_ref().map_while(|x| u32::try_from(*x).ok()).collect();
    assert_eq!(v, &[1, 2]);
    let v: Vec<_> = i.cloned().collect();
    assert_eq!(v, &[4]);
}
