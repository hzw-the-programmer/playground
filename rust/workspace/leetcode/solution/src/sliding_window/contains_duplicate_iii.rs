// 220. Contains Duplicate III

// 时间: O(nk)
// 空间: O(1)
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
// 滑动窗口 + 有序集合
// 时间: O(nlog(min(n,k)))
// 空间: O(min(n,k))
pub fn contains_nearby_almost_duplicate_2(nums: &[i32], k: usize, t: i32) -> bool {
    let n = nums.len();
    let mut set = BTreeSet::new();
    for i in 0..n {
        if let Some(v) = set.range(nums[i] - t..).next() {
            if *v <= nums[i] + t {
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

use std::collections::HashMap;
// 桶排序
// 时间: O(n)
// 空间: O(k)
pub fn contains_nearby_almost_duplicate_3(nums: &[i32], k: usize, t: i32) -> bool {
    let get = |num| {
        if num < 0 {
            // -4, -3, -2, -1 需要放到一个桶，因为 |-4 - (-1)| = |-3| <= 3
            // +1 变成 -3, -2, -1, 0 除以 (3+1)=4 为 0 号桶，但 0 号桶被 0, 1, 2, 3 用了，所以要 -1，得到 -1 号桶
            // -8, -7, -6, -5 需要放到一个桶，因为 |-8 - (-5)| = |-3| <= 3
            // +1 变成 -7, -6, -5, -4 除以 (3+1)=4 为 -1 号桶，但 -1 号桶被 -4, -3, -2, -1 用了，所以要 -1，得到 -2 号桶
            (num + 1) / (t + 1) - 1
        } else {
            // 0, 1, 2, 3 要在 0 号桶，所以是除以 (3+1)=4
            // 4, 5, 6, 7 要在 1 号桶，除以3不行，要除以3+1=4
            num / (t + 1)
        }
    };
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let id = get(nums[i]);
        if map.contains_key(&id) {
            return true;
        }
        // nums = [3, 4]，3在0号桶，4在1号桶
        // nums = [3, 7]，虽然7在1号桶，但7-3>3
        // nums = [3, 8]，8在2号桶，前一个桶1号桶不存在，肯定不满足条件
        if let Some(v) = map.get(&(id - 1)) {
            if nums[i] - *v <= t {
                return true;
            }
        }
        if let Some(v) = map.get(&(id + 1)) {
            if *v - nums[i] <= t {
                return true;
            }
        }
        map.insert(id, nums[i]);
        if i >= k {
            map.remove(&get(nums[i - k]));
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

        assert_eq!(
            true,
            contains_nearby_almost_duplicate_3(&[1, 2, 3, 1], 3, 0)
        );
        assert_eq!(
            false,
            contains_nearby_almost_duplicate_3(&[1, 5, 9, 1, 5, 9], 2, 3)
        );
    }
}
