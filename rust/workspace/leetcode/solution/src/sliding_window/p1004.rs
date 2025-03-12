// 1004. Max Consecutive Ones III

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut zero_count = 0;
    let mut l = 0;
    for r in 0..nums.len() {
        if nums[r] == 0 {
            zero_count += 1;
        }

        while zero_count > k {
            if nums[l] == 0 {
                zero_count -= 1;
            }
            l += 1;
        }

        res = res.max(r - l + 1);
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2));
        assert_eq!(
            10,
            longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            )
        );
    }
}
