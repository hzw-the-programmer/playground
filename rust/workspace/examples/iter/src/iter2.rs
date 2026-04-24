fn test1() {
    let words = vec!["hello world", "rust iterator", "flat map"];

    let chars: Vec<char> = words
        .iter()
        // .flat_map(|&s| s.chars())
        // .flat_map(|s| {
        // expected `i32`, found `&&str`
        // let i: i32 = s;
        // s.chars()
        // })
        .flat_map(|&s| {
            // expected `i32`, found `&str`
            // let i: i32 = s;
            s.chars()
        })
        .collect();

    println!("{:?}", chars);
}

fn test2() {
    let strings = vec!["42", "foo", "7", "bar", "99"];

    let numbers: Vec<i32> = strings
        .into_iter()
        // .filter_map(|s| s.parse().ok())
        .filter_map(|s| {
            // expected `i32`, found `&str`
            // let i: i32 = s;
            s.parse().ok()
        })
        .collect();

    println!("{:?}", numbers);
}

fn test3() {
    let numbers = 1..=5;

    let product = numbers.fold(1, |acc, x| {
        // expected `bool`, found integer
        // let i: bool = x;
        // expected `&i32`, found integer
        // let i: &i32 = x;
        acc * x
    });
    println!("{}", product);
}

fn test4() {
    use std::num::ParseIntError;
    fn test() -> Result<(), ParseIntError> {
        let strings = vec!["10", "20", "xx", "30"];

        let sum: i32 = strings
            .iter()
            .try_fold(0, |acc, s| -> Result<i32, ParseIntError> {
                // expected `i32`, found `&&str`
                // let i: i32 = s;
                let n = s.parse::<i32>()?;
                Ok(acc + n)
            })?; // 若遇到 "xx" 会立即返回 Err

        println!("Sum = {}", sum);
        Ok(())
    }

    let _ = test();
}

fn test5() {
    let numbers = 1..=6;

    let running_sum: Vec<i32> = numbers
        .scan(0, |state, x| {
            // expected `i32`, found `&mut {integer}`
            // let i: i32 = state;
            *state += x;
            Some(*state) // 返回新状态
        })
        .collect();

    println!("{:?}", running_sum); // [1, 3, 6, 10, 15, 21]

    let fib = std::iter::repeat(())
        .scan((0, 1), |(a, b), _| {
            let next = *a + *b;
            let old_a = *a;
            *a = *b;
            *b = next;
            Some(old_a)
        })
        .take(10)
        .collect::<Vec<_>>();
    println!("{:?}", fib); // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
}

fn test6() {
    let names = ["Alice", "Bob", "Charlie"];
    let scores = [100, 80];

    // 自动截断到较短的那个
    let combined: Vec<_> = names.iter().zip(scores.iter()).collect();

    // expected `i32`, found `Vec<(&&str, &{integer})>`
    // let i: i32 = combined;

    // can't compare `(&&str, &{integer})` with `(&str, {integer})`
    // assert_eq!(combined, [("Alice", 100), ("Bob", 80)]);
    assert_eq!(combined, [(&"Alice", &100), (&"Bob", &80)]);
    println!("{:?}", combined); // [("Alice", 100), ("Bob", 80)]
}

fn test7() {
    let colors = ["red", "green", "blue"];
    let infinite_color_cycle = colors.iter().cycle();

    let first_10: Vec<_> = infinite_color_cycle.take(10).collect();

    // expected `i32`, found `Vec<&&str>`
    // let i: i32 = first_10;

    // can't compare `&str` with `str`
    // assert_eq!(first_10, ["red", "green", "blue", "red", "green", "blue", "red", "green", "blue", "red"]);
    assert_eq!(
        first_10,
        [&"red", &"green", &"blue", &"red", &"green", &"blue", &"red", &"green", &"blue", &"red"]
    );

    println!("{:?}", first_10);
    // ["red", "green", "blue", "red", "green", "blue", ...]
}

fn test8() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // let (even, odd): (Vec<&i32>, Vec<_>) = numbers.iter().partition(|&&x| x % 2 == 0);
    let (even, odd): (Vec<i32>, Vec<_>) = numbers.iter().partition(|&&x| x % 2 == 0);

    println!("Even: {:?}, Odd: {:?}", even, odd);
}

pub fn test() {
    test7();
}
