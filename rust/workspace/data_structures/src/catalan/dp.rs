// Time Complexity: O(n^2)
// Auxiliary Space: O(n)
pub fn catalan(n: usize) -> usize {
    if n <= 1 {
        1
    } else {
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            dp[i] = 0;
            for j in 0..i {
                dp[i] += dp[j] * dp[i - j - 1];
            }
        }

        dp[n]
    }
}
