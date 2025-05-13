pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let (mut left, mut right) = (0, n - 1);
    let (mut left_max, mut right_max) = (0, 0);
    let mut ans = 0;
    while left < right {
        left_max = left_max.max(height[left]);
        right_max = right_max.max(height[right]);
        if height[left] < height[right] {
            ans += left_max - height[left];
            left += 1;
        } else {
            ans += right_max - height[right];
            right -= 1;
        }
    }
    ans
}
