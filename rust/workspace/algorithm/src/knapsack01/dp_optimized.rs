// Bottom-Up DP (Space-Optimized)
// time: O(n*capacity)
// space: O(capacity)
pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    assert_eq!(
        weights.len(),
        values.len(),
        "weights and values must have the same length"
    );

    let mut dp = vec![0; capacity + 1];

    for (&w, &v) in weights.iter().zip(values.iter()) {
        for j in (w..=capacity).rev() {
            dp[j] = dp[j].max(v + dp[j - w]);
        }
    }

    dp[capacity]
}

// pub fn knapsack01(weights: &[u32], values: &[u32], capacity: u32) -> u32 {
//     assert_eq!(
//         weights.len(),
//         values.len(),
//         "weights and values must have the same length"
//     );

//     let n = weights.len();
//     let w = capacity as usize;
//     let mut dp = vec![0; w + 1];

//     for i in 1..=n {
//         let mut j = w;
//         while j as u32 >= weights[i - 1] {
//             dp[j] = dp[j].max(values[i - 1] + dp[j - weights[i - 1] as usize]);
//             if j > 0 {
//                 j -= 1;
//             } else {
//                 break;
//             }
//         }
//     }

//     dp[w]
// }
