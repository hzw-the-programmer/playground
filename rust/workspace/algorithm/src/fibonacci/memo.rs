// Top-Down DP (Memoization)
// time: O(n)
// space: O(n)
pub fn fibonacci(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    fibonacci_recursive(n, &mut memo)
}

fn fibonacci_recursive(n: usize, memo: &mut [usize]) -> usize {
    if n <= 1 {
        n
    } else {
        if memo[n] == 0 {
            memo[n] = fibonacci(n - 1) + fibonacci(n - 2);
        }
        memo[n]
    }
}
