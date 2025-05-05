// time: O(n)
// space: O(1)
pub fn count_ways(n: usize) -> usize {
    let mut dp = [1, 1];

    for _ in 2..=n {
        let r = dp[0] + dp[1];
        dp[0] = dp[1];
        dp[1] = r;
    }

    dp[1]
}
