pub fn knapsack01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    assert_eq!(weights.len(), values.len(), "weights and values must have the same length");

    let n = weights.len();

    // base case
    if n == 0 || capacity == 0 {
        return 0;
    }

    let mut pick = 0;
    if weights[n - 1] <= capacity {
        pick = values[n - 1]
            + knapsack01(
                &weights[..n - 1],
                &values[..n - 1],
                capacity - weights[n - 1],
            );
    }

    let not_pick = knapsack01(&weights[..n - 1], &values[..n - 1], capacity);

    pick.max(not_pick)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let weights = [2, 3, 4, 5];
        let values = [3, 4, 5, 6];
        assert_eq!(knapsack01(&weights, &values, 5), 7);
    }

    #[test]
    fn test_medium_case() {
        let weights = [1, 3, 4, 5];
        let values = [1, 4, 5, 7];
        assert_eq!(knapsack01(&weights, &values, 7), 9);
    }

    #[test]
    fn test_empty_items() {
        assert_eq!(knapsack01(&[], &[], 10), 0);
    }

    #[test]
    fn test_zero_capacity() {
        let weights = [1, 2, 3];
        let values = [2, 3, 4];
        assert_eq!(knapsack01(&weights, &values, 0), 0);
    }

    #[test]
    #[should_panic(expected="weights and values must have the same length")]
    fn test_invalid_input() {
        let weights = [1, 2];
        let values = [5];
        knapsack01(&weights, &values, 3);
    }
}
