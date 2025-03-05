use std::collections::HashMap;

// 1852. Distinct Numbers in Each Subarray
pub fn distinct_nums(nums: &[i32], k: usize) -> Vec<usize> {
    let n = nums.len();
    let mut res = vec![0; n - k + 1];

    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(nums[i]).or_insert(0) += 1;
    }
    res[0] = map.len();

    for i in k..n {
        *map.entry(nums[i]).or_insert(0) += 1;

        let c = map.get_mut(&nums[i - k]).unwrap();
        *c -= 1;
        if *c == 0 {
            map.remove(&nums[i - k]);
        }

        res[i - k + 1] = map.len();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            [3, 2, 2, 2, 3][..],
            distinct_nums(&[1, 2, 3, 2, 2, 1, 3], 3)
        );
        assert_eq!([1, 2, 3, 4][..], distinct_nums(&[1, 1, 1, 1, 2, 3, 4], 4));
    }
}
