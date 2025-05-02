// 1852. Distinct Numbers in Each Subarray
// 1 <= k <= nums.length <= 10^5
// 1 <= nums[i] <= 10^5

pub fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut ans = vec![0; n - k + 1];

    let mut map = vec![0; 100_001];
    let mut count = 0;
    for i in 0..n {
        let idx = nums[i] as usize;
        map[idx] += 1;
        if map[idx] == 1 {
            count += 1;
        }

        if i < k - 1 {
            continue;
        }

        ans[i + 1 - k] = count;

        let idx = nums[i + 1 - k] as usize;
        map[idx] -= 1;
        if map[idx] == 0 {
            count -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_numbers() {
        assert_eq!(
            distinct_numbers(vec![1, 2, 3, 2, 2, 1, 3], 3),
            vec![3, 2, 2, 2, 3]
        );
        assert_eq!(
            distinct_numbers(vec![1, 1, 1, 1, 2, 3, 4], 4),
            vec![1, 2, 3, 4]
        );
    }
}
