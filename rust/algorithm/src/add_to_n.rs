fn add_to_n_iterative(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn add_to_n_recursive(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    n + add_to_n_recursive(n - 1)
}

#[test]
fn add_to_n_iterative_test() {
    assert_eq!(add_to_n_iterative(100), 5050);
}

#[test]
fn add_to_n_recursive_test() {
    assert_eq!(add_to_n_recursive(100), 5050);
}
