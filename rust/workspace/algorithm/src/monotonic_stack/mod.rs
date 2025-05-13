pub fn next_greater_element(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut result = vec![-1; n];
    let mut stack = Vec::with_capacity(n);
    for (i, &n) in arr.iter().enumerate() {
        while !stack.is_empty() && n > arr[*stack.last().unwrap()] {
            result[stack.pop().unwrap()] = n;
        }
        stack.push(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_element() {
        assert_eq!(
            next_greater_element(&[2, 1, 2, 4, 3]),
            vec![4, 2, 4, -1, -1]
        );
    }
}
