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

// https://www.geeksforgeeks.org/largest-rectangular-area-in-a-histogram-using-stack/#further-optimized-approach
pub fn largest_rectangle_in_histogram(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut stack = Vec::with_capacity(n);
    let mut ans = 0;
    for (i, &n) in arr.iter().enumerate() {
        while !stack.is_empty() && n < arr[*stack.last().unwrap()] {
            let j = stack.pop().unwrap();
            let h = arr[j];
            let w = if stack.is_empty() {
                i
            } else {
                i - *stack.last().unwrap() - 1
            };
            ans = ans.max(h * w as i32);
        }
        stack.push(i);
    }

    while !stack.is_empty() {
        let j = stack.pop().unwrap();
        let h = arr[j];
        let w = if stack.is_empty() {
            n
        } else {
            n - *stack.last().unwrap() - 1
        };
        ans = ans.max(h * w as i32);
    }

    ans
}

#[cfg(test)]
mod tests;
