// 1343. Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold

pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let threshold = threshold * k;
    let k = k as usize;

    let mut sum = arr[0..k].iter().sum::<i32>();
    let mut ans = if sum >= threshold { 1 } else { 0 };

    for i in k..arr.len() {
        sum += arr[i] - arr[i - k];
        if sum >= threshold {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_subarrays() {
        assert_eq!(3, num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4));
        assert_eq!(
            6,
            num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5)
        );
    }
}
