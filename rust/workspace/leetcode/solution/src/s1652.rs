// 1652. Defuse the Bomb

pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    let mut ans = vec![0; n];
    if k == 0 {
        return ans;
    }
    let mut pos = if k > 0 { n - 1 } else { -k as usize };
    let k = k.abs() as usize;
    let mut sum = 0;

    for i in 0..(2 * n - 1) {
        sum += code[i % n];
        if i < k - 1 {
            continue;
        }
        ans[pos % n] = sum;
        pos += 1;
        sum -= code[(i + 1 - k) % n];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt(vec![5, 7, 1, 4], 3), [12, 10, 16, 13]);
        assert_eq!(decrypt(vec![1, 2, 3, 4], 0), [0, 0, 0, 0]);
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), [12, 5, 6, 13]);
    }
}
