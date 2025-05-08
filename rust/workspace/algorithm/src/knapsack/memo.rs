// time: O(n*capacity)
// space: O(n*capacity)
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

    let mut memo = vec![vec![-1; capacity + 1]; weights.len()];
    knapsack_recursive(weights, values, 0, capacity, &mut memo)
}

fn knapsack_recursive(
    weights: &[usize],
    values: &[usize],
    i: usize,
    capacity: usize,
    memo: &mut [Vec<i32>],
) -> usize {
    if i == weights.len() || capacity == 0 {
        0
    } else {
        if memo[i][capacity] == -1 {
            let mut take = 0;
            let w = weights[i];
            if w > 0 && w <= capacity {
                take = values[i] + knapsack_recursive(weights, values, i, capacity - w, memo);
            }

            let no_take = knapsack_recursive(weights, values, i + 1, capacity, memo);

            memo[i][capacity] = take.max(no_take) as _
        }
        memo[i][capacity] as _
    }
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

//     let mut memo = vec![vec![-1; capacity + 1]; weights.len() + 1];
//     knapsack_recursive(weights, values, weights.len(), capacity, &mut memo)
// }

// fn knapsack_recursive(
//     weights: &[usize],
//     values: &[usize],
//     len: usize,
//     capacity: usize,
//     memo: &mut [Vec<i32>],
// ) -> usize {
//     if len == 0 || capacity == 0 {
//         0
//     } else {
//         if memo[len][capacity] == -1 {
//             let mut take = 0;
//             let w = weights[len - 1];
//             if w > 0 && w <= capacity {
//                 take =
//                     values[len - 1] + knapsack_recursive(weights, values, len, capacity - w, memo);
//             }

//             let no_take = knapsack_recursive(weights, values, len - 1, capacity, memo);

//             memo[len][capacity] = take.max(no_take) as _
//         }
//         memo[len][capacity] as _
//     }
// }
