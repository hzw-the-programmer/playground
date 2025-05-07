// Bottom-Up DP (Tabulation)
// time: O(n*capacity)
// space: O(n*capacity)
pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    assert_eq!(
        weights.len(),
        values.len(),
        "weights and values must have the same length"
    );

    let n = weights.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    // Build table dp[][] in bottom-up manner
    for i in 0..=n {
        for j in 0..=capacity {
            // If there is no item or the knapsack's capacity is 0
            if i == 0 || j == 0 {
                dp[i][j] = 0;
            } else {
                // Pick the ith item if it does not exceed the capacity of knapsack
                let mut pick = 0;
                if weights[i - 1] <= j {
                    pick = values[i - 1] + dp[i - 1][j - weights[i - 1]]
                }
                // Don't pick the ith item
                let not_pick = dp[i - 1][j];
                dp[i][j] = pick.max(not_pick);
            }
        }
    }

    dp[n][capacity]
}
