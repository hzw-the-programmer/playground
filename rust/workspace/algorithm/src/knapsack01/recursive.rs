// time: O(2^n)
// space: O(n)
pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    assert_eq!(
        weights.len(),
        values.len(),
        "weights and values must have the same length"
    );

    knapsack01_recursive(weights, values, weights.len(), capacity)
}

fn knapsack01_recursive(weights: &[usize], values: &[usize], len: usize, capacity: usize) -> usize {
    if len == 0 || capacity == 0 {
        return 0;
    }

    let mut pick = 0;
    if weights[len - 1] <= capacity {
        pick = values[len - 1]
            + knapsack01_recursive(weights, values, len - 1, capacity - weights[len - 1]);
    }

    let not_pick = knapsack01_recursive(weights, values, len - 1, capacity);

    pick.max(not_pick)
}
