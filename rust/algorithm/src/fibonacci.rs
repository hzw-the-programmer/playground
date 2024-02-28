fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut fib_minus_one = 1;
    let mut fib_minus_two = 0;
    let mut fib = 0;
    for _ in 2..=n {
        fib = fib_minus_one + fib_minus_two;
        fib_minus_one = fib;
        fib_minus_two = fib_minus_one;
    }
    fib
}

#[test]
fn fibonacci_test() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
}
