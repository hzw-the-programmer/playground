// 2516. Take K of Each Character From Left and Right

pub fn take_characters(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut res = i32::MAX;
    let mut cnt = [0; 3];
    let mut l = 0;
    for r in 0..2 * n {
        cnt[(s[r % n] - b'a') as usize] += 1;
        while cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
            res = res.min((r - l + 1) as i32);
            cnt[(s[l % n] - b'a') as usize] -= 1;
            l += 1;
        }
    }
    if res == i32::MAX {
        -1
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(take_characters("aabaaaacaabc".to_string(), 2), 8);
        assert_eq!(take_characters("a".to_string(), 1), -1);
    }
}
