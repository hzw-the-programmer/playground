// 1151. Minimum Swaps to Group All 1's Together

pub fn min_swaps(data: Vec<i32>) -> i32 {
    let n = data.len();
    let k = data.iter().filter(|&&n| n == 1).count();
    if k < 2 || k == n {
        return 0;
    }

    let mut ans = i32::MAX;
    let mut count = 0;
    for i in 0..n {
        if data[i] == 0 {
            count += 1;
        }

        if i < k - 1 {
            continue;
        }

        ans = ans.min(count);

        if data[i + 1 - k] == 0 {
            count -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps() {
        assert_eq!(min_swaps(vec![1, 0, 1, 0, 1]), 1);
        assert_eq!(min_swaps(vec![0, 0, 0, 1, 0]), 0);
        assert_eq!(min_swaps(vec![1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1]), 3);
    }
}
