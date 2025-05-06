// time: O(n)
// space: O(n)
pub fn count_ways(n: usize) -> usize {
    if n <= 1 {
        1
    } else {
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }
}
