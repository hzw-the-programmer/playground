pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut memo = vec![2; nums.len()];
    recursive(&nums, nums.len() - 1, &mut memo)
}

fn recursive(nums: &[i32], i: usize, memo: &mut [u8]) -> bool {
    if i == 0 {
        return true;
    }
    if memo[i] != 2 {
        return memo[i] == 1;
    }

    for j in (0..i).rev() {
        if nums[j] >= (i - j) as i32 {
            if recursive(nums, j, memo) {
                memo[i] = 1;
                return true;
            }
        }
    }
    memo[i] = 0;
    false
}
