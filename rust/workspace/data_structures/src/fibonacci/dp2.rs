// time: O(n)
// space: O(1)
pub fn fibonacci(n: usize) -> usize {
    let mut dp = [0, 1];

    for i in 2..=n {
        dp[i % 2] = dp[(i - 2) % 2] + dp[(i - 1) % 2];
    }

    dp[n % 2]
}
