pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut ans = 0;
    let mut stack = Vec::with_capacity(s.len());
    stack.push(-1);
    for (i, c) in s.chars().enumerate() {
        let i = i as i32;
        if c == '(' {
            stack.push(i);
        } else {
            stack.pop();
            if stack.is_empty() {
                stack.push(i);
            } else {
                ans = ans.max(i - stack.last().unwrap());
            }
        }
    }
    ans
}
