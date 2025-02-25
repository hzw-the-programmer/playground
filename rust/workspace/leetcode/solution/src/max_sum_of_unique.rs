// 2841. Maximum Sum of Almost Unique Subarray
use std::collections::HashMap;

pub fn max_sum_of_unique(nums: &[i32], k: usize, m: usize) -> i32 {
    let mut map = HashMap::new();
    let mut sum = 0;
    for i in 0..k {
        // map.entry(nums[i])
        //     .and_modify(|counter| *counter += 1)
        //     .or_insert(1);
        *map.entry(nums[i]).or_insert(0) += 1;
        sum += nums[i];
    }

    let mut max = 0;
    if map.len() >= m {
        max = sum;
    }

    for i in k..nums.len() {
        // if let Some(count) = map.get_mut(&nums[i - k]) {
        //     *count -= 1;
        //     if *count == 0 {
        //         map.remove(&nums[i - k]);
        //     }
        // }
        let count = map.get_mut(&nums[i - k]).unwrap();
        *count -= 1;
        if *count == 0 {
            map.remove(&nums[i - k]);
        }
        // map.entry(nums[i])
        //     .and_modify(|counter| *counter += 1)
        //     .or_insert(1);
        *map.entry(nums[i]).or_insert(0) += 1;
        sum += nums[i] - nums[i - k];
        if map.len() >= m {
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
        assert_eq!(26, max_sum_of_unique(&[5, 8, 8, 9, 9], 3, 2));
        assert_eq!(26, max_sum_of_unique(&[5, 8, 8, 9, 9, 9], 3, 2));
    }
}
