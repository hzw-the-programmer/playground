// time: O((2^2n)n)
// space: O(n)
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut current = vec![0; 2 * n as usize];
    let mut result = vec![];
    generate_parenthesis_recursive(&mut current, 0, &mut result);
    result
}

fn generate_parenthesis_recursive(current: &mut Vec<u8>, pos: usize, result: &mut Vec<String>) {
    if pos == current.len() {
        if valid(current) {
            result.push(unsafe { String::from_utf8_unchecked(current.clone()) });
        }
    } else {
        current[pos] = b'(';
        generate_parenthesis_recursive(current, pos + 1, result);
        current[pos] = b')';
        generate_parenthesis_recursive(current, pos + 1, result);
    }
}

fn valid(current: &[u8]) -> bool {
    let mut balance = 0;
    for &b in current {
        if b == b'(' {
            balance += 1;
        } else if b == b')' {
            balance -= 1;
        }

        if balance < 0 {
            return false;
        }
    }
    balance == 0
}
