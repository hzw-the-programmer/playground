// 1423. Maximum Points You Can Obtain from Cards

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let n = card_points.len();
    let k = n - k as usize;

    if k == 0 {
        return card_points.iter().sum();
    }

    let mut ans = i32::MAX;
    let mut total = 0;
    let mut sum = 0;

    for i in 0..n {
        total += card_points[i];
        sum += card_points[i];
        if i + 1 < k {
            continue;
        }
        ans = ans.min(sum);
        sum -= card_points[i + 1 - k];
    }

    total - ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(max_score(vec![2, 2, 2], 2), 4);
        assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
        assert_eq!(max_score(vec![1, 1000, 1], 1), 1);
        assert_eq!(max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3), 202);
    }
}
