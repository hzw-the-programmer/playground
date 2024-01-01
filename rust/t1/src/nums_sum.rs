fn nums_sum1(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

fn nums_sum2(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        nums[0] + nums_sum2(&nums[1..])
    }
}

fn nums_sum3(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        nums_sum2(&nums[..nums.len() - 1]) + nums[nums.len() - 1]
    }
}

fn nums_sum4(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    } else {
        nums_sum4(sum + nums[0], &nums[1..])
    }
}

fn nums_sum5(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    } else {
        nums_sum5(sum + nums[nums.len() - 1], &nums[..nums.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums_sum1_test() {
        let nums = [2, 1, 7, 4, 5];
        assert_eq!(19, nums_sum1(&nums));
    }

    #[test]
    fn nums_sum2_test() {
        let nums = [2, 1, 7, 4, 5];
        assert_eq!(19, nums_sum2(&nums));
    }

    #[test]
    fn nums_sum3_test() {
        let nums = [2, 1, 7, 4, 5];
        assert_eq!(19, nums_sum3(&nums));
    }

    #[test]
    fn nums_sum4_test() {
        let nums = [2, 1, 7, 4, 5];
        assert_eq!(19, nums_sum4(0, &nums));
    }

    #[test]
    fn nums_sum5_test() {
        let nums = [2, 1, 7, 4, 5];
        assert_eq!(19, nums_sum5(0, &nums));
    }
}
