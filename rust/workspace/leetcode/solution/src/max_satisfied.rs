pub fn max_satisfied(customers: &[i32], grumpy: &[u8], minutes: usize) -> i32 {
    let mut satisfied = 0;
    let mut sum = 0;
    for i in 0..minutes {
        sum += customers[i];
        if grumpy[i] == 0 {
            satisfied += customers[i];
        }
    }

    let mut max = sum;
    let mut index = minutes;
    for i in minutes..customers.len() {
        sum += customers[i] - customers[i - minutes];
        if grumpy[i] == 0 {
            satisfied += customers[i];
        }
        if sum > max {
            max = sum;
            index = i + 1;
        }
    }

    for i in index - minutes..index {
        if grumpy[i] == 1 {
            satisfied += customers[i];
        }
    }

    satisfied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            16,
            max_satisfied(&[1, 0, 1, 2, 1, 1, 7, 5], &[0, 1, 0, 1, 0, 1, 0, 1], 3)
        );
        assert_eq!(1, max_satisfied(&[1], &[0], 1));
    }
}
