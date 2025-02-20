// cd /d/github/hzw-the-programmer/playground/rust/workspace
// cargo test -p valid-parenthese
fn main() {}

fn valid_parenthese(s: &str) -> bool {
    let mut stack = vec![];
    for (i, c) in s.chars().enumerate() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(i);
        } else {
            if let Some(i) = stack.pop() {
                let left = s.chars().nth(i).unwrap();
                if (c == ')' && left != '(')
                    || (c == ']' && left != '[')
                    || (c == '}' && left != '{')
                {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "()";
        assert!(valid_parenthese(s));
    }

    #[test]
    fn test_2() {
        let s = "()[]{}";
        assert!(valid_parenthese(s));
    }

    #[test]
    fn test_3() {
        let s = "(]";
        assert!(!valid_parenthese(s));
    }

    #[test]
    fn test_4() {
        let s = "([])";
        assert!(valid_parenthese(s));
    }
}
