// time: O(2^n)
// space: O(n)
pub fn subset_sum(nums: &[u32], sum: u32) -> bool {
    let n = nums.len();

    if sum == 0 {
        true
    } else if n == 0 {
        false
    } else {
        let mut ans = false;
        if nums[n - 1] <= sum {
            ans = subset_sum(&nums[..n - 1], sum - nums[n - 1]);
        }
        ans || subset_sum(&nums[..n - 1], sum)
    }
}
