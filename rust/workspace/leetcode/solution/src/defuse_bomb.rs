// 1652. Defuse the Bomb
pub fn decrypt(code: &[i32], k: i32) -> Vec<i32> {
    let mut res = vec![0; code.len()];

    if k == 0 {
        return res;
    }

    if k > 0 {
        let k = k as usize;
        for i in 0..code.len() {
            for j in i + 1..i + 1 + k {
                res[i] += code[j % code.len()];
            }
        }
    } else {
        let k = -k as usize;
        for i in 0..code.len() {
            let s = i + code.len() - k;
            for j in s..s + k {
                res[i] += code[j % code.len()];
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
