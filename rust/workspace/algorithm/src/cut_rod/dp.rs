// time: O(n^2)
// space: O(n)
pub fn cut_rod(prices: &[i32], length: usize) -> i32 {
    if length == 0 {
        return 0;
    }

    let mut dp = vec![0; length + 1];

    for i in 1..=length {
        dp[i] = (1..=i)
            .map(|j| prices.get(j - 1).copied().unwrap_or(0) + dp[i - j])
            .max()
            .unwrap_or(0);
    }

    dp[length]
}
