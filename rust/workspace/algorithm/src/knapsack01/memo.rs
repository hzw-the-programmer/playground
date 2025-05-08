// Top-Down DP (Memoization)
// time: O(n*capacity)
// space: O(n*capacity)
pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    assert_eq!(
        weights.len(),
        values.len(),
        "weights and values must have the same length"
    );

    let mut memo = vec![vec![-1; capacity + 1]; weights.len()];
    knapsack01_recursive(weights, values, 0, capacity, &mut memo)
}

fn knapsack01_recursive(
    weights: &[usize],
    values: &[usize],
    i: usize,
    capacity: usize,
    memo: &mut [Vec<i32>],
) -> usize {
    // Base Case
    if i == weights.len() || capacity == 0 {
        return 0;
    }

    // Check if we have previously calculated the same subproblem
    if memo[i][capacity] != -1 {
        return memo[i][capacity] as usize;
    }

    // Pick nth item if it does not exceed the capacity of knapsack
    let mut pick = 0;
    if weights[i] <= capacity {
        pick =
            values[i] + knapsack01_recursive(weights, values, i + 1, capacity - weights[i], memo);
    }

    // Don't pick the nth item
    let not_pick = knapsack01_recursive(weights, values, i + 1, capacity, memo);

    // Store the result in memo[n][capacity] and return it
    memo[i][capacity] = pick.max(not_pick) as i32;
    memo[i][capacity] as usize
}

// pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
//     assert_eq!(
//         weights.len(),
//         values.len(),
//         "weights and values must have the same length"
//     );

//     let mut memo = vec![vec![-1; capacity + 1]; weights.len() + 1];
//     knapsack01_recursive(weights, values, weights.len(), capacity, &mut memo)
// }

// fn knapsack01_recursive(
//     weights: &[usize],
//     values: &[usize],
//     len: usize,
//     capacity: usize,
//     memo: &mut [Vec<i32>],
// ) -> usize {
//     // Base Case
//     if len == 0 || capacity == 0 {
//         return 0;
//     }

//     // Check if we have previously calculated the same subproblem
//     if memo[len][capacity] != -1 {
//         return memo[len][capacity] as usize;
//     }

//     // Pick nth item if it does not exceed the capacity of knapsack
//     let mut pick = 0;
//     if weights[len - 1] <= capacity {
//         pick = values[len - 1]
//             + knapsack01_recursive(weights, values, len - 1, capacity - weights[len - 1], memo);
//     }

//     // Don't pick the nth item
//     let not_pick = knapsack01_recursive(weights, values, len - 1, capacity, memo);

//     // Store the result in memo[n][capacity] and return it
//     memo[len][capacity] = pick.max(not_pick) as i32;
//     memo[len][capacity] as usize
// }
