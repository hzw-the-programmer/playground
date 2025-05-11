pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut temp = vec![];
    let mut result = vec![];
    recursive(&mut temp, 1, n as usize, k as usize, &mut result);
    result
}

fn recursive(temp: &mut Vec<i32>, cur: usize, n: usize, k: usize, result: &mut Vec<Vec<i32>>) {
    if temp.len() + (n + 1 - cur) < k {
        return;
    }

    if temp.len() == k {
        result.push(temp.clone());
        return;
    }

    temp.push(cur as i32);
    recursive(temp, cur + 1, n, k, result);
    temp.pop();

    recursive(temp, cur + 1, n, k, result);
}
