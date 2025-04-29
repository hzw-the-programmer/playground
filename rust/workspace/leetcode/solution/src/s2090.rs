// 2090. K Radius Subarray Averages

pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let len = 2 * k + 1;
    let mut ans = vec![-1; n];

    if n < len {
        return ans;
    }

    let mut sum = 0;
    for i in 0..len {
        sum += nums[i] as i64;
    }
    ans[k] = (sum / len as i64) as i32;

    for i in len..n {
        sum += (nums[i] - nums[i - len]) as i64;
        ans[i - k] = (sum / len as i64) as i32;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_averages() {
        assert_eq!(
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1],
            get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3)
        );
        assert_eq!(vec![100000], get_averages(vec![100000], 0));
        assert_eq!(vec![-1], get_averages(vec![8], 100000));
    }
}
