pub fn max_average(nums: &[i32], k: usize) -> f32 {
    let mut sum = 0;
    for i in 0..k {
        sum += nums[i];
    }

    let mut max = sum;

    for i in k..nums.len() {
        sum += nums[i] - nums[i - k];
        max = max.max(sum);
    }

    max as f32 / k as f32
}

// cargo test max_average -p solution
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(12.75, max_average(&[1, 12, -5, -6, 50, 3], 4));
        assert_eq!(5., max_average(&[5], 1));
    }
}
