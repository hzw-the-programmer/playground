pub fn jump(nums: Vec<i32>) -> i32 {
    let mut jumps = 0;
    let mut end = 0;
    let mut max = 0;

    for (i, &n) in nums[..nums.len() - 1].iter().enumerate() {
        max = max.max(i + n as usize);
        if i == end {
            jumps += 1;
            end = max;
        }
    }

    jumps
}
