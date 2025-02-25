// 2090. K Radius Subarray Average
pub fn k_radius_average(nums: &[i32], k: usize) -> Vec<i32> {
    let total = 2 * k + 1;
    let mut result = vec![-1; nums.len()];

    if nums.len() < total {
        return result;
    }

    let mut sum: i32 = nums[..total].iter().sum();
    result[k] = sum / total as i32;

    for i in k + 1..nums.len() - k {
        sum += nums[i + k] - nums[i - k - 1];
        result[i] = sum / total as i32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            &[-1, -1, -1, 5, 4, 4, -1, -1, -1][..],
            &k_radius_average(&[7, 4, 3, 9, 1, 8, 5, 2, 6], 3)[..]
        );
        assert_eq!(&[100000][..], &k_radius_average(&[100000], 0)[..]);
        assert_eq!(&[-1][..], &k_radius_average(&[8], 100000)[..]);
    }
}
