// 2134. Minimum Swaps to Group All 1's Together II
pub fn min_swaps_2(nums: &[i32]) -> usize {
    let k = nums.iter().filter(|&&x| x == 1).count();
    let mut count = nums[0..k].iter().filter(|&&x| x == 0).count();
    let mut min = count;
    for i in 1..nums.len() {
        if nums[i - 1] == 0 {
            count -= 1;
        }
        if nums[(i + k - 1) % nums.len()] == 0 {
            count += 1;
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
        assert_eq!(1, min_swaps_2(&[0, 1, 0, 1, 1, 0, 0]));
        assert_eq!(2, min_swaps_2(&[0, 1, 1, 1, 0, 0, 1, 1, 0]));
        assert_eq!(0, min_swaps_2(&[1, 1, 0, 0, 1]));
    }
}
