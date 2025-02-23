fn main() {}

fn generate_parentheses(n: u32) -> Vec<String> {
    let mut results = vec![];
    let mut current = String::new();
    backtrack(n, 0, 0, &mut current, &mut results);
    results
}

fn backtrack(n: u32, left: u32, right: u32, current: &mut String, results: &mut Vec<String>) {
    if left == n && right == n {
        results.push(current.clone());
        return;
    }

    if left < n {
        current.push('(');
        backtrack(n, left + 1, right, current, results);
        current.pop();
    }

    if right < left {
        current.push(')');
        backtrack(n, left, right + 1, current, results);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let wanted = vec!["()"];
        assert_eq!(wanted, generate_parentheses(1));
    }

    #[test]
    fn test_2() {
        let wanted = vec!["(())", "()()"];
        assert_eq!(wanted, generate_parentheses(2));
    }

    #[test]
    fn test_3() {
        let wanted = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(wanted, generate_parentheses(3));
    }
}
