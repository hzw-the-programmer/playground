pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let mut jump = 1;
    let mut end = 0 + nums[0] as usize;
    let mut max = 0;
    for i in 1..nums.len() {
        if i > end {
            jump += 1;
            end = max;
        }
        max = max.max(i + nums[i] as usize);
    }
    jump
}
