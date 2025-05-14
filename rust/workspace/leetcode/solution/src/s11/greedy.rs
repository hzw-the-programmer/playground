pub fn max_area(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut ans = 0;
    let (mut left, mut right) = (0, n - 1);
    while left < right {
        if height[left] < height[right] {
            ans = ans.max(height[left] * (right - left) as i32);
            left += 1;
        } else {
            ans = ans.max(height[right] * (right - left) as i32);
            right -= 1;
        }
    }
    ans
}
