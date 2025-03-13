// 1658. Minimum Operations to Reduce X to Zero

pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let target = nums.iter().sum::<i32>() - x;
    if target < 0 {
        return -1;
    } else if target == 0 {
        return nums.len() as i32;
    }

    let mut res = 0;
    let mut sum = 0;
    let mut l = 0;
    for r in 0..nums.len() {
        sum += nums[r];
        while sum > target {
            sum -= nums[l];
            l += 1;
        }
        if sum == target {
            res = res.max(r - l + 1);
        }
    }
    if res == 0 {
        -1
    } else {
        (nums.len() - res) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, min_operations(vec![1, 1, 4, 2, 3], 5));
        assert_eq!(-1, min_operations(vec![5, 6, 7, 8, 9], 4));
        assert_eq!(5, min_operations(vec![3, 2, 20, 1, 1, 3], 10));
        assert_eq!(2, min_operations(vec![2, 3], 5));
    }
}
