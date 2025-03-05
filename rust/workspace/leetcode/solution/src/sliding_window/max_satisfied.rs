// 1052. Grumpy Bookstore Owner
// 只需对不满意最多的那一段施加魔法
pub fn max_satisfied(customers: &[i32], grumpy: &[u8], minutes: usize) -> i32 {
    let mut satisfied = 0;
    let mut increase = 0;
    for i in 0..minutes {
        if grumpy[i] == 0 {
            satisfied += customers[i];
        } else {
            increase += customers[i];
        }
    }

    let mut max_increase = increase;
    for i in minutes..customers.len() {
        if grumpy[i] == 0 {
            satisfied += customers[i];
        } else {
            increase += customers[i];
        }
        if grumpy[i - minutes] == 1 {
            increase -= customers[i - minutes];
        }
        max_increase = max_increase.max(increase);
    }

    satisfied + max_increase
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
        assert_eq!(
            16,
            max_satisfied(&[1, 1, 1, 5, 1, 0, 7, 2], &[0, 1, 0, 1, 0, 1, 0, 1], 3)
        );
    }
}
