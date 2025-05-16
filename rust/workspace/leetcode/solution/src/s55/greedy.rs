pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max = 0;
    for (i, &n) in nums.iter().enumerate() {
        if i > max {
            return false;
        }
        max = max.max(i + n as usize);
        if max >= nums.len() - 1 {
            break;
        }
    }
    true
}
