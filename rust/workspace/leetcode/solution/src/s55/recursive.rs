pub fn can_jump(nums: Vec<i32>) -> bool {
    recursive(&nums, nums.len() - 1)
}

fn recursive(nums: &[i32], i: usize) -> bool {
    if i == 0 {
        return true;
    }
    for j in 0..i {
        if nums[j] >= (i - j) as i32 {
            if recursive(nums, j) {
                return true;
            }
        }
    }
    false
}
