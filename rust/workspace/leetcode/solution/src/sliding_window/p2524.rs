use std::collections::HashMap;

// 2524. Maximum Frequency Score of a Subarray
pub fn max_frequency_score(nums: Vec<i32>, k: usize) -> i32 {
    let n = nums.len();
    let d = 10_i32.pow(9) + 7;
    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(nums[i]).or_insert(0) += 1;
    }

    let mut max = map.iter().map(|(k, v)| k.pow(*v)).sum::<i32>() % d;
    for i in k..n {
        *map.entry(nums[i]).or_insert(0) += 1;
        let v = map.get_mut(&nums[i-k]).unwrap();
        *v -= 1;
        if *v == 0 {
            map.remove(&nums[i-k]);
        }
        max = max.max(map.iter().map(|(k, v)| k.pow(*v)).sum::<i32>() % d);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, max_frequency_score(vec![1,1,1,2,1,2], 3));
        assert_eq!(1, max_frequency_score(vec![1,1,1,1,1,1], 4));
    }
}