pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut ans = 0;

    let (mut left, mut right) = (0, 0);
    for c in s.chars() {
        if c == '(' {
            left += 1;
        } else {
            right += 1;
        }

        if left == right {
            ans = ans.max(2 * left);
        } else if right > left {
            (left, right) = (0, 0);
        }
    }

    (left, right) = (0, 0);
    for c in s.chars().rev() {
        if c == '(' {
            left += 1;
        } else {
            right += 1;
        }

        if left == right {
            ans = ans.max(2 * left);
        } else if right < left {
            (left, right) = (0, 0);
        }
    }

    ans
}
