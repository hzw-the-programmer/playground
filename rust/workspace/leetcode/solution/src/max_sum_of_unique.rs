use std::collections::HashSet;

pub fn max_sum_of_unique(nums: &[i32], k: usize, m: usize) -> i32 {
    let mut set = HashSet::new();
    let mut sum = 0;
    for i in 0..k {
        set.insert(nums[i]);
        sum += nums[i];
    }

    let mut max = 0;
    if set.len() >= m {
        max = sum;
    }

    for i in k..nums.len() {
        set.remove(&nums[i - k]);
        set.insert(nums[i]);
        sum += nums[i] - nums[i - k];
        if set.len() >= m {
            max = max.max(sum);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(18, max_sum_of_unique(&[2, 6, 7, 3, 1, 7], 4, 3));
        assert_eq!(23, max_sum_of_unique(&[5, 9, 9, 2, 4, 5, 4], 3, 1));
        assert_eq!(0, max_sum_of_unique(&[1, 2, 1, 2, 1, 2, 1], 3, 3));
    }
}
