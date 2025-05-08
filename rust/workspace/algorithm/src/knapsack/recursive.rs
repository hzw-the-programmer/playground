// time: O(2^n)
// space: O(capacity)
pub fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    debug_assert_eq!(
        weights
            .iter()
            .zip(values.iter())
            .filter(|&(&w, &v)| w == 0 && v > 0)
            .count(),
        0,
        "Item with weight 0 and positive value is not allowed"
    );

    knapsack_recursive(weights, values, weights.len(), capacity)
}

fn knapsack_recursive(weights: &[usize], values: &[usize], len: usize, capacity: usize) -> usize {
    if len == 0 || capacity == 0 {
        return 0;
    }

    let mut take = 0;
    let w = weights[len - 1];
    if w > 0 && w <= capacity {
        take = values[len - 1] + knapsack_recursive(weights, values, len, capacity - w);
    }

    let no_take = knapsack_recursive(weights, values, len - 1, capacity);

    take.max(no_take)
}

// pub fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
//     debug_assert_eq!(
//         weights
//             .iter()
//             .zip(values.iter())
//             .filter(|&(&w, &v)| w == 0 && v > 0)
//             .count(),
//         0,
//         "Item with weight 0 and positive value is not allowed"
//     );

//     knapsack_recursive(weights, values, 0, capacity)
// }

// fn knapsack_recursive(weights: &[usize], values: &[usize], i: usize, capacity: usize) -> usize {
//     if i == weights.len() || capacity == 0 {
//         return 0;
//     }

//     let mut take = 0;
//     let w = weights[i];
//     if w > 0 && w <= capacity {
//         take = values[i] + knapsack_recursive(weights, values, i, capacity - w);
//     }

//     let no_take = knapsack_recursive(weights, values, i + 1, capacity);

//     take.max(no_take)
// }
