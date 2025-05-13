pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut stack = Vec::with_capacity(n);
    let mut ans = 0;
    for (right, &h) in height.iter().enumerate() {
        while !stack.is_empty() && h > height[*stack.last().unwrap()] {
            let top = stack.pop().unwrap();
            if stack.is_empty() {
                break;
            }
            let left = *stack.last().unwrap();
            let w = right - left - 1;
            let h = h.min(height[left]) - height[top];
            ans += w as i32 * h;
        }
        stack.push(right);
    }
    ans
}
