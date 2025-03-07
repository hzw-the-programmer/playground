// 220. Contains Duplicate III

/*
  时间: O(nk)
  空间: O(1)
*/
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

/*
  滑动窗口 + 有序集合
  时间: O(nlog(min(n,k)))
  空间: O(min(n,k))
*/
use std::collections::BTreeSet;

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

/*
  桶排序
  时间: O(n)
  空间: O(k)

  用商将数分类：
  -3 倍的分一类；-2 倍的分一类；-1 倍的分一类
   0 倍的分一类； 1 倍的分一类； 2 倍的分一类

  设 k=3，那么：
  0, 1,  2,  3 这一组数相互做减法 |a - b| <= 3，要分到一组：x/(k+1)=0 倍组
  4, 5,  6,  7 这一组数相互做减法 |a - b| <= 3，要分到一组：x/(k+1)=1 倍组
  8, 9, 10, 11 这一组数相互做减法 |a - b| <= 3，要分到一组：x/(k+1)=2 倍组
  2 倍组到 0 倍组，不存在 |a - b| <= 3 的情况：8 - 3 = 5
  但，1 倍组到 0 倍组，存在 |a - b| <= 3 的情况：4-3, 4-2, 4-1, 5-3, 5-2, 6-3

  -4, -3, -2, -1 这一组数相互做减法 |a - b| <= 3，要分到一组
  如果按 x/(k+1) 就会有 -4 在 -1 倍组，-3, -2, -1 在 0 倍组
  如果按 (x+1)/(k+1) -4, -3, -2, -1 会在 0 倍组
  但 0 倍组已经被 0, 1, 2, 3 占用了
  所以要按 (x+1)/(k+1) - 1
  -4, -3, -2, -1 在 -1 倍组
  -8, -7, -6, -5 在 -2 倍组
*/
use std::collections::HashMap;

pub fn contains_nearby_almost_duplicate_3(nums: &[i32], k: usize, t: i32) -> bool {
    let get = |num| {
        if num < 0 {
            (num + 1) / (t + 1) - 1
        } else {
            num / (t + 1)
        }
    };
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let id = get(nums[i]);
        if map.contains_key(&id) {
            return true;
        }
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
