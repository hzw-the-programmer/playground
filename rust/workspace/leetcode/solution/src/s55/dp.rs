pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut dp = vec![false; nums.len()];
    dp[0] = true;
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[j] >= (i - j) as i32 {
                dp[i] = dp[j];
            }
        }
    }
    dp[nums.len() - 1]
}
