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

pub fn decrypt_v2(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    let mut ans = vec![0; n];
    if k == 0 {
        return ans;
    }

    let mut start = if k > 0 { 1 } else { (n as i32 + k) as usize };
    let mut end = if k > 0 { k as usize } else { n - 1 };
    let mut sum = 0;
    for i in start..=end {
        sum += code[i];
    }

    for i in 0..n {
        ans[i] = sum;
        sum -= code[start];
        start = (start + 1) % n;
        end = (end + 1) % n;
        sum += code[end];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(f: fn(Vec<i32>, i32) -> Vec<i32>) {
        assert_eq!(f(vec![5, 7, 1, 4], 3), [12, 10, 16, 13]);
        assert_eq!(f(vec![1, 2, 3, 4], 0), [0, 0, 0, 0]);
        assert_eq!(f(vec![2, 4, 9, 3], -2), [12, 5, 6, 13]);
    }

    #[test]
    fn test_decrypt() {
        test(decrypt);
    }

    #[test]
    fn test_decrypt_v2() {
        test(decrypt_v2);
    }
}
