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
    // map_while_4();

    // skip();

    // take();
    // take_2();
    // take_3();

    // scan();

    // flat_map();

    // flatten();
    // flatten_2();
    // flatten_3();
    // flatten_4();
    // flatten_5();
    // flatten_6();

    // map_windows();
    // map_windows_2();
    // map_windows_3();
    // map_windows_4();
    // map_windows_5();
    // map_windows_6();

    // fuse();

    // inspect();
    // inspect_2();

    // by_ref();

    // collect();

    // try_collect();

    // collect_into();

    // partition();

    // partition_in_place();

    // is_partitioned();

    // try_fold();

    // try_for_each();

    // fold();

    // reduce();

    // try_reduce();

    // all();

    // find();

    // find_map();

    // try_find();

    // position();

    // rposition();

    // max();

    // min();

    // max_by_key();

    // max_by();

    // min_by_key();

    // min_by();

    // rev();

    // unzip();

    // copied();

    // cloned();

    // cycle();

    // next_chunk();

    // size_hint();

    // count();

    // last();

    // advance_by();

    // nth();

    // step_by();

    // chain();

    // zip();

    intersperse();
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

fn skip() {
    let a = [1, 2, 3];
    let mut i = a.iter().skip(2);
    assert_eq!(Some(&3), i.next());
    assert_eq!(None, i.next());
}

fn take() {
    let a = [1, 2, 3];
    let mut i = a.iter().take(2);
    assert_eq!(Some(&1), i.next());
    assert_eq!(Some(&2), i.next());
    assert_eq!(None, i.next());
}

fn take_2() {
    let mut i = (0..).take(3);
    assert_eq!(Some(0), i.next());
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(None, i.next());
}

fn take_3() {
    let v = [1, 2];
    let mut i = v.into_iter().take(5);
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(None, i.next());
}

fn scan() {
    let a = [1, 2, 3, 4];
    let mut i = a.iter().scan(1, |s, &x| {
        *s = *s * x;
        if *s > 6 {
            None
        } else {
            Some(-*s)
        }
    });
    assert_eq!(i.next(), Some(-1));
    assert_eq!(i.next(), Some(-2));
    assert_eq!(i.next(), Some(-6));
    assert_eq!(i.next(), None);
}

fn flat_map() {
    let a = ["apha", "beta", "gamma"];
    let v: String = a.iter().flat_map(|s| s.chars()).collect();
    assert_eq!(v, "aphabetagamma");
}

fn flatten() {
    let a = vec![vec![1, 2, 3, 4], vec![5, 6]];
    let v = a.into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(v, &[1, 2, 3, 4, 5, 6]);
}

fn flatten_2() {
    let a = ["alpha", "beta", "gamma"];
    let s: String = a.iter().map(|s| s.chars()).flatten().collect();
    assert_eq!(s, "alphabetagamma");
}

fn flatten_3() {
    let a = [Some(123), Some(321), None, Some(231)];
    let v: Vec<_> = a.iter().flatten().collect();
    assert_eq!(v, &[&123, &321, &231]);
}

fn flatten_4() {
    let a = [Some(123), Some(321), None, Some(231)];
    let v: Vec<_> = a.into_iter().flatten().collect();
    assert_eq!(v, &[123, 321, 231]);
}

fn flatten_5() {
    let a = [Ok(123), Ok(321), Err(456), Ok(231)];
    let v: Vec<_> = a.iter().flatten().collect();
    assert_eq!(v, &[&123, &321, &231]);
}

fn flatten_6() {
    let a = [Ok(123), Ok(321), Err(456), Ok(231)];
    let v: Vec<_> = a.into_iter().flatten().collect();
    assert_eq!(v, &[123, 321, 231]);
}

fn map_windows() {
    let a = "abcd"
        .chars()
        .map_windows(|[x, y]| {
            // expected `i32`, found `&char`
            // let i: i32 = x;
            format!("{x}+{y}")
        })
        .collect::<Vec<_>>();
    assert_eq!(a, &["a+b", &"b+c", &"c+d"]);
}

fn map_windows_2() {
    let v = "abcde"
        .chars()
        .map_windows(|&[x, y, z]| {
            // expected `i32`, found `char`
            // let i: i32 = x;
            format!("{x}+{y}+{z}")
        })
        .collect::<Vec<_>>();
    assert_eq!(v, &["a+b+c", "b+c+d", "c+d+e"]);
}

fn map_windows_3() {
    // let mut i = [1, 3, 8, 1].iter().map_windows(|[a, b]| {
    let mut i = [1, 3, 8, 1].iter().map_windows(|&[a, b]| {
        // expected `i32`, found `&&{integer}`
        // expected `i32`, found `&{integer}`
        // let i: i32 = a;
        a + b
    });
    assert_eq!(i.next(), Some(4));
    assert_eq!(i.next(), Some(11));
    assert_eq!(i.next(), Some(9));
    assert_eq!(i.next(), None);
}

fn map_windows_4() {
    let mut i = "ferris".chars().map_windows(|w: &[_; 3]| *w);
    assert_eq!(i.next(), Some(['f', 'e', 'r']));
    assert_eq!(i.next(), Some(['e', 'r', 'r']));
    assert_eq!(i.next(), Some(['r', 'r', 'i']));
    assert_eq!(i.next(), Some(['r', 'i', 's']));
    assert_eq!(i.next(), None);
}

fn map_windows_5() {
    let mut i = [0.5, 1.0, 3.5, 3.0, 8.5, 8.5, f32::NAN]
        .iter()
        .map_windows(|[a, b]| {
            // expected `i32`, found `&&f32`
            // let i: i32 = a;
            a <= b
        });
    assert_eq!(i.next(), Some(true));
    assert_eq!(i.next(), Some(true));
    assert_eq!(i.next(), Some(false));
    assert_eq!(i.next(), Some(true));
    assert_eq!(i.next(), Some(true));
    assert_eq!(i.next(), Some(false));
    assert_eq!(i.next(), None);
}

fn map_windows_6() {
    #[derive(Default)]
    struct NonFusedIterator {
        state: i32,
    }

    impl Iterator for NonFusedIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let v = self.state;
            self.state += 1;
            if v < 5 || v % 2 == 0 {
                Some(v)
            } else {
                None
            }
        }
    }

    let mut i = NonFusedIterator::default();
    assert_eq!(i.next(), Some(0));
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), Some(2));
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), Some(4));
    assert_eq!(i.next(), None);

    assert_eq!(i.next(), Some(6));
    assert_eq!(i.next(), None);

    assert_eq!(i.next(), Some(8));
    assert_eq!(i.next(), None);

    let mut i = NonFusedIterator::default().map_windows(|a: &[_; 2]| *a);
    assert_eq!(i.next(), Some([0, 1]));
    assert_eq!(i.next(), Some([1, 2]));
    assert_eq!(i.next(), Some([2, 3]));
    assert_eq!(i.next(), Some([3, 4]));
    assert_eq!(i.next(), None);

    assert_eq!(i.next(), None);
    assert_eq!(i.next(), None);
    assert_eq!(i.next(), None);
}

fn fuse() {
    struct Alternate {
        state: i32,
    }
    impl Iterator for Alternate {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            let v = self.state;
            self.state += 1;
            if v % 2 == 0 {
                Some(v)
            } else {
                None
            }
        }
    }

    let mut i = Alternate { state: 0 };
    assert_eq!(i.next(), Some(0));
    assert_eq!(i.next(), None);
    assert_eq!(i.next(), Some(2));
    assert_eq!(i.next(), None);

    let mut i = i.fuse();
    assert_eq!(i.next(), Some(4));
    assert_eq!(i.next(), None);
    assert_eq!(i.next(), None);
    assert_eq!(i.next(), None);
}

fn inspect() {
    let a = [1, 4, 2, 3];
    let sum = a
        .iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, x| sum + x);
    println!("{sum}");
    let sum = a
        .iter()
        .cloned()
        .inspect(|x| println!("about to filter: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {x}"))
        .fold(0, |sum, x| sum + x);
    println!("{sum}");
}

fn inspect_2() {
    let a = ["1", "2", "a", "b", "5"];
    let sum = a
        .iter()
        .map(|l| l.parse::<i32>())
        .inspect(|r| {
            // expected `i32`, found `&Result<i32, ParseIntError>`
            // let i: i32 = r;
            if let Err(e) = r {
                // expected `i32`, found `&ParseIntError`
                // let i: i32 = e;
                println!("Parsing err: {e}");
            }
        })
        .filter_map(Result::ok)
        .sum::<i32>();
    println!("{sum}");
}

fn by_ref() {
    let mut words = ["hello", "world", "of", "Rust"].into_iter();
    let hello_world: Vec<_> = words.by_ref().take(2).collect();
    assert_eq!(hello_world, &["hello", "world"]);
    let of_rust: Vec<_> = words.collect();
    assert_eq!(of_rust, &["of", "Rust"]);
}

fn collect() {
    let a = [1, 2, 3];
    let doubled: Vec<_> = a.iter().map(|&x| x * 2).collect();
    assert_eq!(doubled, &[2, 4, 6]);

    use std::collections::VecDeque;

    let doubled: VecDeque<_> = a.iter().map(|&x| x * 2).collect();
    assert_eq!(doubled[0], 2);
    assert_eq!(doubled[1], 4);
    assert_eq!(doubled[2], 6);

    let doubled = a.iter().map(|&x| x * 2).collect::<Vec<_>>();
    assert_eq!(doubled, &[2, 4, 6]);

    let chars = ['g', 'd', 'k', 'k', 'n'];
    let hello: String = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect();
    assert_eq!(hello, "hello");

    let r = [Ok(1), Err("nope"), Ok(3), Err("bad")];
    let r: Result<Vec<_>, &str> = r.iter().cloned().collect();
    assert_eq!(r, Err("nope"));

    let r = [Ok(1), Ok(3)];
    let r: Result<Vec<_>, &str> = r.iter().cloned().collect();
    assert_eq!(r, Ok(vec![1, 3]));

    let o = [Some(1), None, Some(3), None];
    let o: Option<Vec<_>> = o.iter().cloned().collect();
    assert_eq!(o, None);

    let o = [Some(1), Some(2)];
    let o: Option<Vec<_>> = o.iter().cloned().collect();
    assert_eq!(o, Some(vec![1, 2]));
}

fn try_collect() {
    let o = [Some(1), Some(2), Some(3)];
    let o = o.iter().cloned().try_collect::<Vec<_>>();
    assert_eq!(o, Some(vec![1, 2, 3]));

    let o = [Some(1), None, Some(3)];
    let o = o.into_iter().try_collect::<Vec<_>>();
    assert_eq!(o, None);

    // let r: [Result<_, ()>; 3] = [Ok(1), Ok(2), Ok(3)];
    let r = [Ok::<_, ()>(1), Ok(2), Ok(3)];
    let r = r.iter().cloned().try_collect::<Vec<_>>();
    assert_eq!(r, Ok(vec![1, 2, 3]));

    let r = vec![Ok(1), Err(()), Ok(2)];
    let r = r.into_iter().try_collect::<Vec<_>>();
    assert_eq!(r, Err(()));

    use core::ops::ControlFlow::{Break, Continue};
    let c = [Continue(1), Continue(2), Break(3), Continue(4), Continue(5)];
    let mut i = c.into_iter();
    let c = i.try_collect::<Vec<_>>();
    assert_eq!(c, Break(3));
    let c = i.try_collect::<Vec<_>>();
    assert_eq!(c, Continue(vec![4, 5]));
}

fn collect_into() {
    let a = [1, 2, 3];
    let mut v = vec![0, 1];
    a.iter().map(|&x| x * 2).collect_into(&mut v);
    assert_eq!(v, &[0, 1, 2, 4, 6]);
    a.iter().map(|&x| x * 10).collect_into(&mut v);
    assert_eq!(v, &[0, 1, 2, 4, 6, 10, 20, 30]);

    let mut v = Vec::with_capacity(6);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 6);
    a.iter().map(|&x| x * 2).collect_into(&mut v);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 6);
    assert_eq!(v, &[2, 4, 6]);
    a.iter().map(|&x| x * 10).collect_into(&mut v);
    assert_eq!(v.len(), 6);
    assert_eq!(v.capacity(), 6);
    assert_eq!(v, &[2, 4, 6, 10, 20, 30]);

    // type annotations needed for `Vec<_>`
    // let mut v = Vec::with_capacity(6);
    // associated function takes 0 generic arguments but 1 generic argument was supplied
    // let mut v = Vec::with_capacity::<i32>(6);
    let mut v: Vec<i32> = Vec::with_capacity(6);
    let count = a.iter().collect_into(&mut v).iter().count();
    assert_eq!(v.len(), count);
    assert_eq!(v, &[1, 2, 3]);
    let count = a.iter().collect_into(&mut v).iter().count();
    assert_eq!(v.len(), count);
    assert_eq!(v, &[1, 2, 3, 1, 2, 3]);
}

fn partition() {
    let a = [1, 2, 3];
    let (even, odd): (Vec<_>, Vec<_>) = a.into_iter().partition(|n| {
        // expected `i32`, found `&{integer}`
        // let i: i32 = n;
        n % 2 == 0
    });
    assert_eq!(even, &[2]);
    assert_eq!(odd, &[1, 3]);
}

fn partition_in_place() {
    let mut a = [1, 2, 3, 4, 5, 6, 7];
    let l = a.iter_mut().partition_in_place(|n| {
        // expected `i32`, found `&{integer}`
        // let i: i32 = n;
        n % 2 == 0
    });
    assert_eq!(a[..l], [6, 2, 4]);
    assert_eq!(a[l..], [3, 5, 1, 7]);
}

fn is_partitioned() {
    assert!("Iterator".chars().is_partitioned(char::is_uppercase));
    assert!(!"IntoIterator".chars().is_partitioned(char::is_uppercase));
}

fn try_fold() {
    let a = [1, 2, 3];
    let sum = a.iter().try_fold(0i8, |acc, &x| acc.checked_add(x));
    assert_eq!(sum, Some(6));

    let a = [10, 20, 30, 100, 40, 50];
    let mut i = a.iter();
    let sum = i.try_fold(0i8, |acc, &n| acc.checked_add(n));
    assert_eq!(sum, None);
    assert_eq!(i.len(), 2);
    assert_eq!(i.next(), Some(&40));

    use core::ops::ControlFlow;
    let mut i = 1..30;
    let triangular = i.try_fold(0_i8, |acc, x| {
        // expected `char`, found integer
        // let i: char = x;
        if let Some(acc) = acc.checked_add(x) {
            ControlFlow::Continue(acc)
        } else {
            ControlFlow::Break(acc)
        }
    });
    assert_eq!(triangular, ControlFlow::Break(120));
    assert_eq!(i.next(), Some(17));

    let mut i = 1..30;
    let triangular = i.try_fold(0_u64, |pre, x| {
        if let Some(next) = pre.checked_add(x) {
            ControlFlow::Continue(next)
        } else {
            ControlFlow::Break(pre)
        }
    });
    assert_eq!(triangular, ControlFlow::Continue(435));
    assert_eq!(i.next(), None);
}

fn try_for_each() {
    use std::fs::rename;
    use std::io::{stdout, Write};
    use std::path::Path;
    let data = ["no_tea.txt", "stable_bread.json", "torrential_rain.png"];
    let r = data.iter().try_for_each(|x| {
        // expected `i32`, found `&&str`
        // let i: i32 = x;
        writeln!(stdout(), "{x}")
    });
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), ());

    let mut i = data.iter();
    let r = i.try_for_each(|x| rename(x, Path::new(x).with_extension("old")));
    assert!(r.is_err());
    assert_eq!(i.next(), Some(&"stable_bread.json"));

    let mut i = data.iter().cloned();
    let r = i.try_for_each(|x| rename(x, Path::new(x).with_extension("old")));
    assert!(r.is_err());
    assert_eq!(i.next(), Some("stable_bread.json"));

    use core::ops::ControlFlow;
    let r = (2..100).try_for_each(|x| {
        if 323 % x == 0 {
            ControlFlow::Break(x)
        } else {
            ControlFlow::Continue(())
        }
    });
    assert_eq!(r, ControlFlow::Break(17));
}

fn fold() {
    let a = [1, 2, 3];
    let sum = a.iter().fold(0, |acc, x| {
        // expected `i32`, found `&{integer}`
        // let i: i32 = x;
        acc + x
    });
    assert_eq!(sum, 6);

    let a = [1, 2, 3, 4, 5];
    let zero = "0".to_string();
    let sum = a.iter().fold(zero, |acc, &x| format!("({acc} + {x})"));
    assert_eq!(sum, "(((((0 + 1) + 2) + 3) + 4) + 5)");

    let a = [1, 2, 3, 4, 5];
    let zero = "0".to_string();
    let sum = a.iter().fold(zero, |acc, x| format!("({acc} + {x})"));
    assert_eq!(sum, "(((((0 + 1) + 2) + 3) + 4) + 5)");

    let a = [1, 2, 3, 4, 5];
    let mut result = 0;
    for i in &a {
        result += i;
    }
    assert_eq!(result, 15);

    let r = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(r, result);

    let r = a.iter().fold(0, |acc, &x| acc + x);
    assert_eq!(r, result);
}

fn reduce() {
    let r = (1..10).reduce(|acc, x| acc + x).unwrap();
    assert_eq!(r, 45);

    let r = (1..10).fold(0, |acc, x| acc + x);
    assert_eq!(r, 45);
}

fn try_reduce() {
    let n = vec![10usize, 20, 5, 23, 0];
    let r = n.into_iter().try_reduce(|x, y| x.checked_add(y));
    assert_eq!(r, Some(Some(58)));

    let n = vec![1, 2, usize::MAX, 4, 5];
    let mut i = n.into_iter();
    let r = i.try_reduce(|x, y| x.checked_add(y));
    assert_eq!(r, None);
    assert_eq!(i.next(), Some(4));

    let n: Vec<usize> = vec![];
    let r = n.into_iter().try_reduce(|x, y| x.checked_add(y));
    assert_eq!(r, Some(None));

    let n = ["1", "2", "3", "4", "5"];
    let r: Result<_, <usize as core::str::FromStr>::Err> = n.into_iter().try_reduce(|x, y| {
        if x.parse::<usize>()? > y.parse::<usize>()? {
            Ok(x)
        } else {
            Ok(y)
        }
    });
    assert_eq!(r, Ok(Some("5")));

    let n = ["1", "2", "Nan", "4", "5"];
    let mut i = n.into_iter();
    let r: Result<_, <usize as core::str::FromStr>::Err> = i.try_reduce(|x, y| {
        if x.parse::<usize>()? > y.parse::<usize>()? {
            Ok(x)
        } else {
            Ok(y)
        }
    });
    println!("{r:?}");
}

fn all() {
    let a = [1, 2, 3];
    assert!(a.iter().all(|&x| x > 0));
    assert!(!a.iter().all(|&x| x > 2));

    let mut i = a.iter();
    assert!(!i.all(|&x| x != 2));
    assert_eq!(i.next(), Some(&3));
}

fn any() {
    let a = [1, 2, 3];
    assert!(a.iter().any(|&x| x > 0));
    assert!(!a.iter().any(|&x| x > 5));

    let mut i = a.iter();
    assert!(i.any(|&x| x != 2));
    assert_eq!(i.next(), Some(&2));
}

fn find() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().find(|&&x| x == 4), None);

    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.find(|&&x| x == 2), Some(&2));
    assert_eq!(iter.next(), Some(&3));
}

fn find_map() {
    let a = ["lol", "NaN", "2", "5"];
    let n = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(n, Some(2));
}

fn try_find() {
    let a = ["1", "2", "lol", "NaN", "5"];
    let r = a
        .iter()
        .try_find(|&&s| Ok::<_, core::num::ParseIntError>(s.parse::<i32>()? == 2));
    assert_eq!(r, Ok(Some(&"2")));

    let r = a
        .iter()
        .try_find(|s| Ok::<_, core::num::ParseIntError>(s.parse::<i32>()? == 5));
    assert!(r.is_err());

    use core::num::NonZeroU32;
    let a = [3, 5, 7, 4, 9, 0, 11];
    let mut i = a.iter();
    let r = i.try_find(|&&x| NonZeroU32::new(x).map(|y| y.is_power_of_two()));
    assert_eq!(r, Some(Some(&4)));
    assert_eq!(i.next(), Some(&9));

    let mut i = a.iter().take(3);
    let r = i.try_find(|&&x| NonZeroU32::new(x).map(|y| y.is_power_of_two()));
    assert_eq!(r, Some(None));
    assert_eq!(i.next(), None);

    let mut i = a.iter().rev();
    let r = i.try_find(|&&x| NonZeroU32::new(x).map(|y| y.is_power_of_two()));
    assert_eq!(r, None);
    assert_eq!(i.next(), Some(&9));
}

fn position() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().position(|&x| x == 2), Some(1));
    assert_eq!(a.iter().position(|&x| x == 5), None);

    let a = [1, 2, 3, 4];
    let mut i = a.iter();
    assert_eq!(i.position(|x| *x >= 2), Some(1));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.position(|x| *x == 4), Some(0));
}

fn rposition() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().rposition(|&x| x == 3), Some(2));
    assert_eq!(a.iter().rposition(|&x| x == 5), None);

    let a = [1, 2, 3, 4];
    let mut i = a.iter();
    assert_eq!(i.rposition(|&x| x == 4), Some(3));
    assert_eq!(i.rposition(|&x| x == 2), Some(1));
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), None);
}

fn max() {
    let a = [2.4, f32::NAN, 1.3];
    assert_eq!(a.into_iter().reduce(f32::max), Some(2.4));

    let a = [1, 2, 3];
    assert_eq!(a.iter().max(), Some(&3));

    // let a: Vec<u32> = Vec::new();
    let a: [u32; 0] = [];
    assert_eq!(a.iter().max(), None);
}

fn min() {
    let a = [2.4, f32::NAN];
    assert_eq!(a.into_iter().reduce(f32::min), Some(2.4));

    let a = [1, 2, 3];
    assert_eq!(a.iter().min(), Some(&1));

    let a: Vec<u32> = vec![];
    assert_eq!(a.iter().min(), None);
}

fn max_by_key() {
    let a = [-3_i32, 0, 5, -10];
    assert_eq!(a.iter().max_by_key(|x| x.abs()), Some(&-10));
    assert_eq!(*a.iter().max_by_key(|x| x.abs()).unwrap(), -10);
}

fn max_by() {
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(a.iter().max_by(|x, y| x.cmp(y)), Some(&5));
}

fn min_by_key() {
    let a = [-3_i32, 0, 1, 5, 10];
    assert_eq!(a.iter().min_by_key(|x| x.abs()), Some(&0));
}

fn min_by() {
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(a.iter().min_by(|x, y| x.cmp(y)), Some(&-10));
}

fn rev() {
    let a = [1, 2, 3];
    let mut i = a.iter().rev();
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), None);
}

fn unzip() {
    let a = [(1, 2), (3, 4), (5, 6)];
    let (l, r): (Vec<_>, Vec<_>) = a.iter().cloned().unzip();
    assert_eq!(l, [1, 3, 5]);
    assert_eq!(r, [2, 4, 6]);

    let a = [(1, (2, 3)), (4, (5, 6))];
    let (x, (y, z)): (Vec<_>, (Vec<_>, Vec<_>)) = a.iter().cloned().unzip();
    assert_eq!(x, [1, 4]);
    assert_eq!(y, [2, 5]);
    assert_eq!(z, [3, 6]);
}

fn copied() {
    let a = [1, 2, 3];
    let v_copied: Vec<_> = a.iter().copied().collect();
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();
    assert_eq!(v_copied, v_map);
}

fn cloned() {
    let a = [1, 2, 3];
    let v_cloned: Vec<_> = a.iter().cloned().collect();
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();
    assert_eq!(v_cloned, v_map);

    let a = [vec![0, 1, 2], vec![3, 4], vec![23]];
    let slower: Vec<_> = a.iter().cloned().filter(|e| e.len() == 1).collect();
    let faster: Vec<_> = a.iter().filter(|e| e.len() == 1).cloned().collect();
    assert_eq!(slower, faster);
}

fn cycle() {
    let a = [1, 2, 3];
    let mut i = a.iter().cycle();
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), Some(&1));
}

fn next_chunk() {
    let mut a = "lorem".chars();
    assert_eq!(a.next_chunk().unwrap(), ['l', 'o']);
    assert_eq!(a.next_chunk().unwrap(), ['r', 'e', 'm']);
    assert_eq!(a.next_chunk::<4>().unwrap_err().as_slice(), []);

    let quote = "not all those who wander are lost";
    let mut i = quote.split_whitespace();
    let [f, s, t] = i.next_chunk().unwrap();
    assert_eq!(f, "not");
    assert_eq!(s, "all");
    assert_eq!(t, "those");
    assert_eq!(i.next(), Some("who"));
}

fn size_hint() {
    let a = [1, 2, 3];
    let mut i = a.iter();
    assert_eq!(i.size_hint(), (3, Some(3)));
    i.next();
    assert_eq!(i.size_hint(), (2, Some(2)));

    let i = (0..10).filter(|x| x % 2 == 0);
    assert_eq!(i.size_hint(), (0, Some(10)));

    let i = (0..10).filter(|x| x % 2 == 0).chain(15..20);
    assert_eq!(i.size_hint(), (5, Some(15)));

    let i = 0..;
    assert_eq!(i.size_hint(), (usize::MAX, None));
}

fn count() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().count(), 3);
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().count(), 5);

    // cargo run -p iter
    // thread 'main' panicked at /rustc/dbce3b43b6cb34dd3ba12c3ec6f708fe68e9c3df\library\core\src\iter\range.rs:417:1:
    // attempt to add with overflow
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\iter.exe` (exit code: 101)
    // let a = 0..;
    // a.count();
}

fn last() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().last(), Some(&3));
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().last(), Some(&5));
}

fn advance_by() {
    let a = [1, 2, 3, 4];
    let mut i = a.iter();
    assert_eq!(i.advance_by(2), Ok(()));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.advance_by(0), Ok(()));
    use core::num::NonZeroUsize;
    assert_eq!(i.advance_by(100), Err(NonZeroUsize::new(99).unwrap()));

    let a = [Foo(0), Foo(1), Foo(2), Foo(3)];
    let mut i = a.into_iter();
    let _ = i.advance_by(2);
    println!("finish");
}

fn nth() {
    let a = [1, 2, 3];
    let mut i = a.iter();
    assert_eq!(i.nth(0), Some(&1));

    i = a.iter();
    assert_eq!(i.nth(1), Some(&2));
    assert_eq!(i.nth(1), None);

    i = a.iter();
    assert_eq!(i.nth(100), None);

    let a = [Foo(0), Foo(1), Foo(2), Foo(3)];
    let mut i = a.into_iter();
    {
        let e = i.nth(1);
        println!("{e:?}");
    }
    println!("finish");
}

fn step_by() {
    let a = [0, 1, 2, 3, 4, 5];
    let mut i = a.iter().step_by(2);
    assert_eq!(i.next(), Some(&0));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&4));
    assert_eq!(i.next(), None);
}

fn chain() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let mut i = a.iter().chain(b.iter());
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), Some(&4));
    assert_eq!(i.next(), Some(&5));
    assert_eq!(i.next(), Some(&6));
    assert_eq!(i.next(), None);

    let mut i = a.iter().chain(&b);
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), Some(&4));
    assert_eq!(i.next(), Some(&5));
    assert_eq!(i.next(), Some(&6));
    assert_eq!(i.next(), None);
}

fn zip() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let mut i = a.iter().zip(b.iter());
    assert_eq!(i.next(), Some((&1, &4)));
    assert_eq!(i.next(), Some((&2, &5)));
    assert_eq!(i.next(), Some((&3, &6)));
    assert_eq!(i.next(), None);

    let mut i = a.iter().zip(b);
    assert_eq!(i.next(), Some((&1, 4)));
    assert_eq!(i.next(), Some((&2, 5)));
    assert_eq!(i.next(), Some((&3, 6)));
    assert_eq!(i.next(), None);

    let mut i = b.iter();
    assert_eq!(i.next(), Some(&4));

    {
        let b = [Foo(0), Foo(1), Foo(2), Foo(3)];
        let mut i = a.iter().zip(&b);
        assert_eq!(i.next(), Some((&1, &Foo(0))));
        println!("0");
        assert_eq!(i.next(), Some((&2, &Foo(1))));
        println!("1");
        assert_eq!(i.next(), Some((&3, &Foo(2))));
        println!("2");
        assert_eq!(i.next(), None);
        println!("before leave");
    }
    println!("after leave\n");

    {
        let b = [Foo(0), Foo(1), Foo(2), Foo(3)];
        let mut i = a.iter().zip(b);
        // borrow of moved value: `b`
        // let i = b.iter();
        assert_eq!(i.next(), Some((&1, Foo(0))));
        println!("0");
        assert_eq!(i.next(), Some((&2, Foo(1))));
        println!("1");
        assert_eq!(i.next(), Some((&3, Foo(2))));
        println!("2");
        assert_eq!(i.next(), None);
        println!("before leave");
    }
    println!("after leave\n");

    let e: Vec<_> = "foo".chars().enumerate().collect();
    let z: Vec<_> = (0..).zip("foo".chars()).collect();
    assert_eq!(e[0], (0, 'f'));
    assert_eq!(z[0], (0, 'f'));
    assert_eq!(e[1], (1, 'o'));
    assert_eq!(z[1], (1, 'o'));
    assert_eq!(e[2], (2, 'o'));
    assert_eq!(z[2], (2, 'o'));

    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let mut i = core::iter::zip(
        a.into_iter().map(|x| x * 2).skip(1),
        b.into_iter().map(|x| x * 2).skip(1),
    );
    assert_eq!(i.next(), Some((4, 10)));
    assert_eq!(i.next(), Some((6, 12)));
    assert_eq!(i.next(), None);

    let mut i = a
        .iter()
        .map(|x| x * 2)
        .skip(1)
        .zip(b.iter().map(|x| x * 2).skip(1));
    assert_eq!(i.next(), Some((4, 10)));
    assert_eq!(i.next(), Some((6, 12)));
    assert_eq!(i.next(), None);
}

fn intersperse() {
    let mut i = [0, 1, 2].iter().intersperse(&100);
    assert_eq!(i.next(), Some(&0));
    assert_eq!(i.next(), Some(&100));
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&100));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), None);
    assert_eq!(i.next(), None);

    let r: String = ["hello", "world", "!"]
        .iter()
        .copied()
        .intersperse(" ")
        .collect();
    assert_eq!(r, "hello world !");
}

#[derive(Debug, PartialEq)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.0);
    }
}
