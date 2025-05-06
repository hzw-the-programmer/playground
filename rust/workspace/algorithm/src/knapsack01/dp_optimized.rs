pub fn knapsack01(weights: &[u32], values: &[u32], capacity: u32) -> u32 {
    assert_eq!(
        weights.len(),
        values.len(),
        "weights and values must have the same length"
    );

    let n = weights.len();
    let w = capacity as usize;
    let mut dp = vec![0; w + 1];

    for i in 1..n + 1 {
        let mut j = w;
        while j as u32 >= weights[i - 1] {
            dp[j] = dp[j].max(values[i - 1] + dp[j - weights[i - 1] as usize]);
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
    }

    dp[w]
}
