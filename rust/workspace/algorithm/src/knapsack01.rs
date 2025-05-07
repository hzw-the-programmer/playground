// https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/

// mod dp_optimized;
// pub use dp_optimized::knapsack01;

// mod recursive;
// pub use recursive::knapsack01;

mod memo;
pub use memo::knapsack01;

#[cfg(test)]
mod tests;

// Bottom-Up DP (Tabulation)
// time: O(n*capacity)
// space: O(n*capacity)
pub fn knapsack01_dp(weights: &[usize], values: &[usize], capacity: usize) -> usize {
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

// Bottom-Up DP (Space-Optimized)
// time: O(n*capacity)
// space: O(capacity)
pub fn knapsack01_dp_v2(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![0; capacity + 1];
    for i in 1..=n {
        let mut j = capacity;
        while j >= weights[i - 1] {
            dp[j] = dp[j].max(values[i - 1] + dp[j - weights[i - 1]]);
            j -= 1;
        }
    }
    dp[capacity]
}
