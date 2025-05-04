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

pub fn knapsack01_memo(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let mut memo = vec![vec![-1; capacity + 1]; weights.len() + 1];
    knapsack01_memo_recursive(weights, values, capacity, &mut memo)
}

fn knapsack01_memo_recursive(
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
            + knapsack01(
                &weights[..n - 1],
                &values[..n - 1],
                capacity - weights[n - 1],
            );
    }

    // Don't pick the nth item
    let not_pick = knapsack01(&weights[..n - 1], &values[..n - 1], capacity);

    // Store the result in memo[n][capacity] and return it
    memo[n][capacity] = pick.max(not_pick) as i32;
    memo[n][capacity] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let weights = [2, 3, 4, 5];
        let values = [3, 4, 5, 6];
        assert_eq!(knapsack01(&weights, &values, 5), 7);
        assert_eq!(knapsack01_memo(&weights, &values, 5), 7);
    }

    #[test]
    fn test_medium_case() {
        let weights = [1, 3, 4, 5];
        let values = [1, 4, 5, 7];
        assert_eq!(knapsack01(&weights, &values, 7), 9);
        assert_eq!(knapsack01_memo(&weights, &values, 7), 9);
    }

    #[test]
    fn test_empty_items() {
        assert_eq!(knapsack01(&[], &[], 10), 0);
        assert_eq!(knapsack01_memo(&[], &[], 10), 0);
    }

    #[test]
    fn test_zero_capacity() {
        let weights = [1, 2, 3];
        let values = [2, 3, 4];
        assert_eq!(knapsack01(&weights, &values, 0), 0);
        assert_eq!(knapsack01_memo(&weights, &values, 0), 0);
    }

    #[test]
    #[should_panic(expected = "weights and values must have the same length")]
    fn test_invalid_input() {
        let weights = [1, 2];
        let values = [5];
        knapsack01(&weights, &values, 3);
    }
}
