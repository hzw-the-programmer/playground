// time: O(2^n)
// space: O(capacity)
pub fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    knapsack_recursive(weights, values, 0, capacity)
}

fn knapsack_recursive(weights: &[usize], values: &[usize], i: usize, capacity: usize) -> usize {
    if i == weights.len() {
        return 0;
    }

    let mut take = 0;
    if weights[i] <= capacity {
        take = values[i] + knapsack_recursive(weights, values, i, capacity - weights[i]);
    }

    let no_take = knapsack_recursive(weights, values, i + 1, capacity);

    take.max(no_take)
}
