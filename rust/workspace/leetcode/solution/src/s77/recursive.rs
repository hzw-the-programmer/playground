pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut current = vec![];
    let mut result = vec![];
    recursive(&mut current, 1, n as usize, k as usize, &mut result);
    result
}

fn recursive(current: &mut Vec<i32>, i: usize, n: usize, k: usize, result: &mut Vec<Vec<i32>>) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }
    if i > n {
        return;
    }

    current.push(i as i32);
    recursive(current, i + 1, n, k, result);
    current.pop();

    recursive(current, i + 1, n, k, result);
}
