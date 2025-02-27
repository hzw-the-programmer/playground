// 1652. Defuse the Bomb
pub fn decrypt(code: &[i32], k: i32) -> Vec<i32> {
    let n = code.len();
    let mut res = vec![0; n];

    if k == 0 {
        return res;
    }

    let mut start = if k > 0 { 1 } else { (n as i32 + k) as usize };
    let mut end = if k > 0 { k as usize } else { n - 1 };

    let mut sum = 0;
    for i in start..=end {
        sum += code[i];
    }

    for i in 0..n {
        res[i] = sum;

        sum -= code[start];
        start = (start + 1) % n;
        end = (end + 1) % n;
        sum += code[end];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!([12, 10, 16, 13][..], decrypt(&[5, 7, 1, 4], 3));
        assert_eq!([0, 0, 0, 0][..], decrypt(&[1, 2, 3, 4], 0));
        assert_eq!([12, 5, 6, 13][..], decrypt(&[2, 4, 9, 3], -2));
    }
}
