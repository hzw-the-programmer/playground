pub fn num_of_subarrays(nums: &[i32], k: usize, threshold: i32) -> u32 {
    let threshold = threshold * k as i32;
    let mut sum: i32 = nums[..k].iter().sum();
    let mut count = 0;

    if sum >= threshold {
        count += 1;
    }

    for i in k..nums.len() {
        sum += nums[i] - nums[i - k];
        if sum >= threshold {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, num_of_subarrays(&[2, 2, 2, 2, 5, 5, 5, 8], 3, 4));
        assert_eq!(
            6,
            num_of_subarrays(&[11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5)
        );
    }
}
