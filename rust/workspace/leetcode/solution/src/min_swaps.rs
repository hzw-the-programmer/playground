// 1151. Minimum Swaps to Group All 1’s Together
pub fn min_swaps(nums: &[i32]) -> usize {
    let n = nums.len();
    let k = nums.iter().filter(|&&x| x == 1).count();
    if k <= 1 || k == n {
        return 0;
    }

    let mut count = nums[0..k].iter().filter(|&&x| x == 0).count();

    let mut min = count;
    for i in k..n {
        if nums[i] == 0 {
            count += 1;
        }
        if nums[i - k] == 0 {
            count -= 1;
        }
        min = min.min(count);
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, min_swaps(&[1, 0, 1, 0, 1]));
        assert_eq!(0, min_swaps(&[0, 0, 0, 1, 0]));
        assert_eq!(3, min_swaps(&[1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1]));
    }
}
