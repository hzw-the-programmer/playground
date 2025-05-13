pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut left_height = vec![0; n];
    let mut right_height = vec![0; n];

    left_height[0] = height[0];
    for i in 1..n {
        left_height[i] = height[i].max(left_height[i - 1]);
    }

    right_height[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right_height[i] = height[i].max(right_height[i + 1]);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += left_height[i].min(right_height[i]) - height[i];
    }
    ans
}
