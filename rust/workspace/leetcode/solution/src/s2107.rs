// 2107. Number of Unique Flavors After Sharing K Candies
// 1 <= candies[i] <= 10^5
// 0 <= k <= candies.length

pub fn share_candies(candies: Vec<i32>, k: i32) -> i32 {
    let n = candies.len();
    let k = k as usize;

    if k == n {
        return 0;
    }

    let mut map = vec![0; 100_001];
    let mut count = 0;
    for i in 0..n {
        let idx = candies[i] as usize;
        map[idx] += 1;
        if map[idx] == 1 {
            count += 1;
        }
    }

    if k == 0 {
        return count;
    }

    let mut ans = 0;
    for i in 0..n {
        let idx = candies[i] as usize;
        map[idx] -= 1;
        if map[idx] == 0 {
            count -= 1;
        }

        if i < k - 1 {
            continue;
        }

        ans = ans.max(count);

        let idx = candies[i + 1 - k] as usize;
        map[idx] += 1;
        if map[idx] == 1 {
            count += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_share_candies() {
        assert_eq!(share_candies(vec![1, 2, 2, 3, 4, 3], 3), 3);
        assert_eq!(share_candies(vec![2, 2, 2, 2, 3, 3], 2), 2);
        assert_eq!(share_candies(vec![2, 4, 5], 0), 3);

        assert_eq!(share_candies(vec![1, 2, 3, 3, 5, 6], 2), 4);
    }
}
