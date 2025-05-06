// Bottom-Up DP (Tabulation)
// time: O(n)
// space: O(n)
pub fn fibonacci(n: usize) -> usize {
    if n <= 1 {
        n
    } else {
        let mut dp = vec![0; n + 1];
        dp[0] = 0;
        dp[1] = 1;

        for i in 2..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }
}
