// https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/

// mod dp_optimized;
// pub use dp_optimized::knapsack01;

// mod recursive;
// pub use recursive::knapsack01;

// mod memo;
// pub use memo::knapsack01;

mod dp;
pub use dp::knapsack01;

#[cfg(test)]
mod tests;

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
