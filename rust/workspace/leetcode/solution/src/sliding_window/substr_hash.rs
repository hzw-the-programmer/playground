// 2156. Find Substring With Given Hash Value
pub fn substr_hash(s: &str, power: i32, modulo: i32, k: usize, hash_val: i32) -> &str {
    let s = s.as_bytes();
    let n = s.len();
    let val = |b| (b - b'`') as i32;
    let mut cur = 0;
    let mut res = 0;
    let mut pk = 1;
    for i in (0..n).rev() {
        cur = (cur * power + val(s[i])) % modulo;
        if i < n - k {
            cur = (cur + modulo - val(s[i + k]) * pk % modulo) % modulo;
        } else {
            pk = pk * power % modulo;
        }
        if cur == hash_val {
            res = i;
        }
    }
    unsafe { std::str::from_utf8_unchecked(&s[res..res + k]) }
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
