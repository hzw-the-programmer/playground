// time: O(2^n)
// space: O(n)
pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    assert_eq!(
        weights.len(),
        values.len(),
        "weights and values must have the same length"
    );

    let n = weights.len();

    // Base Case
    if n == 0 || capacity == 0 {
        return 0;
    }

    // Pick nth item if it does not exceed the capacity of knapsack
    let mut pick = 0;
    if weights[n - 1] <= capacity {
        pick = values[n - 1]
            + knapsack01(
                &weights[..n - 1],
                &values[..n - 1],
                capacity - weights[n - 1],
            );
    }

    // Don't pick the nth item
    let not_pick = knapsack01(&weights[..n - 1], &values[..n - 1], capacity);

    pick.max(not_pick)
}
