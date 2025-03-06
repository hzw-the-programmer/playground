pub fn contains_nearby_almost_duplicate(nums: &[i32], index_diff: i32, value_diff: i32) -> bool {
    let n = nums.len();
    let k = index_diff as usize + 1;
    for i in 0..k - 1 {
        for j in i + 1..k {
            if (nums[i] - nums[j]).abs() <= value_diff {
                return true;
            }
        }
    }
    for i in k..n {
        for j in i - k + 1..i {
            if (nums[i] - nums[j]).abs() <= value_diff {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, contains_nearby_almost_duplicate(&[1, 2, 3, 1], 3, 0));
        assert_eq!(
            false,
            contains_nearby_almost_duplicate(&[1, 5, 9, 1, 5, 9], 2, 3)
        );
    }
}
