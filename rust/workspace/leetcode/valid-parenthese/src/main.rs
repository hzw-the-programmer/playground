// cd /d/github/hzw-the-programmer/playground/rust/workspace
// cargo test -p valid-parenthese
fn main() {}

fn valid_parenthese(s: &str) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => return false,
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
