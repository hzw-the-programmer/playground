// 1052. Grumpy Bookstore Owner

// 1 <= minutes <= n <= 2 * 104

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let n = customers.len();
    let minutes = minutes as usize;

    let mut unsatisfied = 0;
    let mut max = 0;
    let mut satisfied = 0;
    for i in 0..n {
        if grumpy[i] == 1 {
            unsatisfied += customers[i];
        } else {
            satisfied += customers[i];
        }

        if i < minutes - 1 {
            continue;
        }

        max = max.max(unsatisfied);

        if grumpy[i + 1 - minutes] == 1 {
            unsatisfied -= customers[i + 1 - minutes];
        }
    }

    satisfied + max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_satisfied() {
        assert_eq!(
            max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
        assert_eq!(max_satisfied(vec![1], vec![0], 1), 1);
    }
}
