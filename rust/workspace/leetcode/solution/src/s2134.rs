// 2134. Minimum Swaps to Group All 1's Together II

pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let k = nums.iter().filter(|&&n| n == 1).count();
    if k <= 1 || k == n {
        return 0;
    }

    let mut count = 0;
    let mut ans = i32::MAX;
    for i in 0..n + k - 1 {
        if nums[i % n] == 0 {
            count += 1;
        }

        if i < k - 1 {
            continue;
        }

        ans = ans.min(count);

        if nums[(i + 1 - k) % n] == 0 {
            count -= 1;
        }
    }

    ans
}

pub fn min_swaps_v2(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let k = nums.iter().sum::<i32>() as usize;
    if k <= 1 || k == n {
        return 0;
    }

    let mut zeros = k as i32 - nums[0..k].iter().sum::<i32>();
    let mut ans = zeros;

    for i in 1..n {
        zeros += nums[i - 1] - nums[(i + k - 1) % n];
        ans = ans.min(zeros);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(f: fn(Vec<i32>) -> i32) {
        assert_eq!(f(vec![0, 1, 0, 1, 1, 0, 0]), 1);
        assert_eq!(f(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
        assert_eq!(f(vec![1, 1, 0, 0, 1]), 0);
    }

    #[test]
    fn test_min_swaps() {
        test(min_swaps);
    }

    #[test]
    fn test_min_swaps_v2() {
        test(min_swaps_v2);
    }
}
