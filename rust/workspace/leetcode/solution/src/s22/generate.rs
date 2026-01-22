pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut memo = vec![vec![]; n + 1];
    generate(n, &mut memo);
    memo.remove(n)
}

fn generate(n: usize, memo: &mut Vec<Vec<String>>) -> &Vec<String> {
    let result = &mut memo[n];
    if result.len() != 0 {
        return result;
    }

    if n == 0 {
        result.push("".to_string());
        return result;
    }

    for i in 0..n {
        for left in generate(i, memo) {
            for right in generate(n - 1 - i, memo) {
                result.push(format!("({left}{right})"));
            }
        }
    }

    result
}
