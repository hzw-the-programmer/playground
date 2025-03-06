// 220. Contains Duplicate III
pub fn contains_nearby_almost_duplicate_1(nums: &[i32], index_diff: i32, value_diff: i32) -> bool {
    let n = nums.len();
    let k = index_diff as usize + 1;
    for i in 0..k - 1 {
        for j in i + 1..k {
            if (nums[i] - nums[j]).abs() <= value_diff {
                return true;
            }
        }
    }
    for i in k..n {
        for j in i - k + 1..i {
            if (nums[i] - nums[j]).abs() <= value_diff {
                return true;
            }
        }
    }
    false
}

use std::collections::BTreeSet;

pub fn contains_nearby_almost_duplicate_2(nums: &[i32], index_diff: i32, value_diff: i32) -> bool {
    let n = nums.len();
    let k = index_diff as usize;
    let mut set = BTreeSet::new();
    for i in 0..n {
        if let Some(v) = set.range(nums[i] - value_diff..).next() {
            if *v <= nums[i] + value_diff {
                return true;
            }
        }

        set.insert(nums[i]);

        if i >= k {
            set.remove(&nums[i - k]);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            true,
            contains_nearby_almost_duplicate_1(&[1, 2, 3, 1], 3, 0)
        );
        assert_eq!(
            false,
            contains_nearby_almost_duplicate_1(&[1, 5, 9, 1, 5, 9], 2, 3)
        );

        assert_eq!(
            true,
            contains_nearby_almost_duplicate_2(&[1, 2, 3, 1], 3, 0)
        );
        assert_eq!(
            false,
            contains_nearby_almost_duplicate_2(&[1, 5, 9, 1, 5, 9], 2, 3)
        );
    }
}
