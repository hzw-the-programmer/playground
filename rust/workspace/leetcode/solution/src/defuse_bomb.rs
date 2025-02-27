// 1652. Defuse the Bomb
pub fn decrypt(code: &[i32], k: i32) -> Vec<i32> {
    let n = code.len();
    let mut res = vec![0; n];

    if k == 0 {
        return res;
    }

    if k > 0 {
        for i in 0..n {
            for j in 1..=k as usize {
                res[i] += code[(i + j) % n];
            }
        }
    } else {
        for i in 0..n {
            for j in 1..=-k as usize {
                res[i] += code[(i + n - j) % n];
            }
        }
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
