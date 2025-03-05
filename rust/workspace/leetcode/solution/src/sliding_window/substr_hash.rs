// 2156. Find Substring With Given Hash Value
pub fn substr_hash(s: &str, power: i32, modulo: i32, k: usize, hash_val: i32) -> &str {
    let s = s.as_bytes();
    let n = s.len();
    let mut hash = 0;
    let mut p = 1;
    for i in (0..k).rev() {
        hash = hash * power + (s[i] - b'a' + 1) as i32;
        if i != 0 {
            p *= power;
        }
    }
    if hash % modulo == hash_val {
        unsafe {
            return std::str::from_utf8_unchecked(&s[0..k]);
        }
    }

    for i in k..n {
        hash -= (s[i - k] - b'a' + 1) as i32;
        hash /= power;
        hash += (s[i] - b'a' + 1) as i32 * p;
        if hash % modulo == hash_val {
            unsafe {
                return std::str::from_utf8_unchecked(&s[i - k + 1..=i]);
            }
        }
    }

    unsafe { std::str::from_utf8_unchecked(&s[0..0]) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("ee", substr_hash("leetcode", 7, 20, 2, 0));
        assert_eq!("fbx", substr_hash("fbxzaad", 31, 100, 3, 32));
    }
}
