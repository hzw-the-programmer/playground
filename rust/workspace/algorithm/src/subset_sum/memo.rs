// time: O(sum*n)
// space: O(sum*n)
pub fn subset_sum(nums: &[u32], sum: u32) -> bool {
    let n = nums.len();
    let mut memo = vec![vec![-1; sum as usize + 1]; n + 1];
    subset_sum_recursive(nums, n, sum, &mut memo)
}

fn subset_sum_recursive(nums: &[u32], n: usize, sum: u32, memo: &mut Vec<Vec<i8>>) -> bool {
    if sum == 0 {
        true
    } else if n == 0 {
        false
    } else {
        if memo[n][sum as usize] == -1 {
            let mut res = false;
            if nums[n - 1] <= sum {
                res = subset_sum_recursive(nums, n - 1, sum - nums[n - 1], memo);
            }
            memo[n][sum as usize] = (res || subset_sum_recursive(nums, n - 1, sum, memo)) as i8;
        }
        memo[n][sum as usize] != 0
    }
}
