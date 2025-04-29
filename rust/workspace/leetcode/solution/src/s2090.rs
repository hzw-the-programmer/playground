// 2090. K Radius Subarray Averages

pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let k2 = 2 * k + 1;
    let mut ans = vec![-1; n];

    if n < k2 {
        return ans;
    }

    let mut sum = nums[0..k2].iter().map(|&n| n as i64).sum::<i64>();
    ans[k] = (sum / k2 as i64) as i32;

    for i in k2..n {
        sum += (nums[i] - nums[i - k2]) as i64;
        ans[i - k] = (sum / k2 as i64) as i32;
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
