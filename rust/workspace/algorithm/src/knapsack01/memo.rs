// Top-Down DP (Memoization)
// time: O(n*capacity)
// space: O(n*capacity)
pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    assert_eq!(
        weights.len(),
        values.len(),
        "weights and values must have the same length"
    );

    let mut memo = vec![vec![-1; capacity + 1]; weights.len() + 1];
    knapsack01_recursive(weights, values, capacity, &mut memo)
}

fn knapsack01_recursive(
    weights: &[usize],
    values: &[usize],
    capacity: usize,
    memo: &mut [Vec<i32>],
) -> usize {
    let n = weights.len();

    // Base Case
    if n == 0 || capacity == 0 {
        return 0;
    }

    // Check if we have previously calculated the same subproblem
    if memo[n][capacity] != -1 {
        return memo[n][capacity] as usize;
    }

    // Pick nth item if it does not exceed the capacity of knapsack
    let mut pick = 0;
    if weights[n - 1] <= capacity {
        pick = values[n - 1]
            + knapsack01_recursive(
                &weights[..n - 1],
                &values[..n - 1],
                capacity - weights[n - 1],
                memo,
            );
    }

    // Don't pick the nth item
    let not_pick = knapsack01_recursive(&weights[..n - 1], &values[..n - 1], capacity, memo);

    // Store the result in memo[n][capacity] and return it
    memo[n][capacity] = pick.max(not_pick) as i32;
    memo[n][capacity] as usize
}
