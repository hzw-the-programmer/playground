fn fibonacci_iterative(n: u64) -> u64 {
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

fn fibonacci_recursive(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

#[test]
fn fibonacci_iterative_test() {
    assert_eq!(fibonacci_iterative(0), 0);
    assert_eq!(fibonacci_iterative(1), 1);
    assert_eq!(fibonacci_iterative(2), 1);
}

#[test]
fn fibonacci_recursive_test() {
    assert_eq!(fibonacci_recursive(0), 0);
    assert_eq!(fibonacci_recursive(1), 1);
    assert_eq!(fibonacci_recursive(2), 1);
}
