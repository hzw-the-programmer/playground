// 2831. Find the Longest Equal Subarray

use std::collections::HashMap;

pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, (usize, usize, usize, usize)> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(e) = map.get_mut(&n) {
            *e = (
                i,
                e.1 + i - e.0 - 1,
                e.2 + 1,
                if i == e.0 + 1 { e.3 + 1 } else { e.3 },
            );
        } else {
            map.insert(n, (i, 0, 1, 1));
        }
    }
    let mut res = 0;
    for v in map.values() {
        if v.1 <= k as usize {
            res = res.max(v.2);
        } else {
            res = res.max(v.3);
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3);
        assert_eq!(longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 4);
        assert_eq!(longest_equal_subarray(vec![3, 1, 5, 3, 1, 1], 0), 2);
    }
}
