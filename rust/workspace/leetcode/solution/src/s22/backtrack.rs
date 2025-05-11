pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut current = Vec::with_capacity(2 * n);
    let mut result = Vec::new();
    backtrack(&mut current, n, 0, 0, &mut result);
    result
}

fn backtrack(current: &mut Vec<u8>, n: usize, open: usize, close: usize, result: &mut Vec<String>) {
    if current.len() == 2 * n {
        result.push(unsafe { String::from_utf8_unchecked(current.clone()) });
        return;
    }

    if open < n {
        current.push(b'(');
        backtrack(current, n, open + 1, close, result);
        current.pop();
    }

    if close < open {
        current.push(b')');
        backtrack(current, n, open, close + 1, result);
        current.pop();
    }
}
