fn test1() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens: Vec<_> = numbers
        .iter()
        .filter(|n| {
            // error[E0308]: mismatched types
            // expected `i32`, found `&&{integer}`
            // let i: i32 = n;

            // error[E0369]: cannot calculate the remainder of `&&{integer}` divided by `{integer}`
            // help: `%` can be used on `&{integer}` if you dereference the left-hand side
            // n % 2 == 0

            *n % 2 == 0
        })
        .collect();

    // error[E0277]: can't compare `&{integer}` with `{integer}`
    // assert_eq!(evens, [2, 4, 6, 8, 10]);

    assert_eq!(evens, [&2, &4, &6, &8, &10]);

    println!("{:?}", evens);
}

fn test2() {
    let numbers = vec![1, 2, 3, 4, 5];

    let squares: Vec<_> = numbers
        .iter()
        .map(|n| {
            // error[E0308]: mismatched types
            // expected `i32`, found `&{integer}`
            // let i: i32 = n;

            // error[E0308]: mismatched types
            // expected `&i32`, found integer
            // let i: &i32 = n * n;

            n * n
        })
        .collect();

    assert_eq!(squares, [1, 4, 9, 16, 25]);

    println!("{:?}", squares);
}

fn test3() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<_> = numbers
        .iter()
        // .filter(|&n| n % 2 == 0)
        .filter(|&&n| n % 2 == 0)
        // .map(|n| n * n)
        .map(|&n| n * n)
        .collect();

    assert_eq!(result, [4, 16, 36, 64, 100]);

    println!("{:?}", result);
}

fn test4() {
    let numbers = vec![1, 2, 3, 4, 5];

    // error[E0277]: a value of type `Vec<i32>` cannot be built from an iterator over elements of type `&{integer}`
    // value of type `Vec<i32>` cannot be built from `std::iter::Iterator<Item=&{integer}>`
    // let first_three: Vec<i32> = numbers.iter().take(3).collect();

    let first_three: Vec<_> = numbers.iter().take(3).collect();

    // error[E0277]: can't compare `&{integer}` with `{integer}`
    // no implementation for `&{integer} == {integer}`
    // assert_eq!(first_three, [1, 2, 3]);

    assert_eq!(first_three, [&1, &2, &3]);

    println!("{:?}", first_three);

    let skip_two: Vec<_> = numbers.iter().skip(2).collect();
    assert_eq!(skip_two, [&3, &4, &5]);
    println!("{:?}", skip_two);

    for (index, value) in numbers.iter().enumerate() {
        // error[E0308]: mismatched types
        // expected `i32`, found `&{integer}`
        // let i: i32 = value;

        println!("{}: {}", index, value);
    }

    let letters = vec!['a', 'b', 'c'];
    let paired: Vec<_> = numbers.iter().zip(letters.iter()).collect();
    assert_eq!(paired, [(&1, &'a'), (&2, &'b'), (&3, &'c')]);
    println!("{:?}", paired);
}

fn test5() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<_> = numbers
        .iter()
        .map(|n| {
            // error[E0308]: mismatched types
            // expected `i32`, found `&{integer}`
            // let i: i32 = n;
            n * 2
        })
        .collect();

    println!("{:?}", doubled);

    /*
    ```rust
      impl<'a> Sum<&'a i32> for i32
      fn sum<I>(iter: I) -> i32
      where
          I: Iterator<Item = &'a i32>,
    ```

    ```rust
      impl Sum for i32
      fn sum<I>(iter: I) -> i32
      where
          I: Iterator<Item = i32>,
    ```
    */
    let sum: i32 = numbers.iter().sum();
    println!("{}", sum);

    // error[E0277]: a value of type `i64` cannot be made by multiplying all elements of type `&i32` from an iterator
    // value of type `i64` cannot be made by multiplying all elements from a `std::iter::Iterator<Item=&i32>`
    // help: the trait `Product<&i32>` is not implemented for `i64`
    // help: the following other types implement trait `Product<A>`:
    //        `i64` implements `Product<&i64>`
    //        `i64` implements `Product`
    // let product: i64 = numbers.iter().product();
    let product: i32 = numbers.iter().product();
    println!("{}", product);

    let count = numbers
        .iter()
        .filter(|&n| {
            /*
                fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘ
                where
                    Self: Sized,
                    P: FnMut(&Self::Item) -> bool,
            */

            // error[E0308]: mismatched types
            // expected `i32`, found `&i32`
            // let i: i32 = n;

            /*
                impl Div<&i32> for &i32
                type Output = <i32 as Div>::Output
                fn div(self, other: &i32) -> <i32 as Div>::Output

                impl Div<&i32> for i32
                type Output = <i32 as Div>::Output
                fn div(self, other: &i32) -> <i32 as Div>::Output

                impl Div<i32> for &i32
                type Output = <i32 as Div>::Output
                fn div(self, other: i32) -> <i32 as Div>::Output

                impl Div for i32
                type Output = i32
                fn div(self, other: i32) -> i32
            */
            n % 2 == 0
        })
        .count();
    println!("{}", count);

    let _has_even = numbers.iter().any(|n| {
        /*
            fn any<F>(&mut self, f: F) -> bool
            where
                Self: Sized,
                F: FnMut(Self::Item) -> bool,
        */

        // error[E0308]: mismatched types
        // expected `i32`, found `&i32`
        // let i: i32 = n;

        n % 2 == 0
    });

    let _all_positive = numbers.iter().all(|n| {
        // error[E0308]: mismatched types
        // expected `i32`, found `&i32`
        // let i: i32 = n;

        // error[E0308]: mismatched types
        // expected `&i32`, found integer
        // n > 0

        // n > &0
        *n > 0
    });

    let _first_even = numbers.iter().find(|n| {
        // error[E0308]: mismatched types
        // expected `i32`, found `&&i32`
        // let i: i32 = n;

        // error[E0369]: cannot calculate the remainder of `&&i32` divided by `{integer}`
        // help: `%` can be used on `&i32` if you dereference the left-hand side
        // n % 2 == 0

        *n % 2 == 0
    });
}

fn test6() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut iter = numbers.iter();
    assert_eq!(iter.next(), Some(&1));
    let sum: i32 = iter.sum();
    assert_eq!(sum, 14);
    // assert_eq!(iter.next(), None);
}

fn test7() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<_> = numbers.iter().map(|n| n * 2).collect();

    // error[E0308]: mismatched types
    // expected `bool`, found integer
    // let b: bool = result[0];
    println!("{:?}", result);
}

fn test8() {
    let threshhold = 5;
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<_> = numbers.iter().filter(|&n| n > &threshhold).collect();

    println!("{:?}", result);
}

fn test9() {
    fn t(_v: Vec<i32>) {}

    let numbers = vec![1, 2, 3, 4, 5];
    let iter = numbers.iter();
    // t(numbers);
    // drop(numbers);
    for n in iter {
        println!("{n}")
    }
}

pub fn test() {
    test9();
}
