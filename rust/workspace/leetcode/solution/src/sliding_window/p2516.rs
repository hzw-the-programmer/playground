// 2516. Take K of Each Character From Left and Right

pub fn take_characters(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();

    let mut cnt1 = [0; 3];
    for &b in s.iter() {
        cnt1[(b - b'a') as usize] += 1;
    }
    if cnt1[0] < k || cnt1[1] < k || cnt1[2] < k {
        return -1;
    } else if cnt1[0] == k && cnt1[1] == k && cnt1[2] == k {
        return n as _;
    }

    for c in &mut cnt1 {
        *c -= k;
    }

    let mut res = 0;
    let mut cnt2 = [0; 3];
    let mut l = 0;
    for r in 0..n {
        let i = (s[r] - b'a') as usize;
        cnt2[i] += 1;
        while cnt2[i] > cnt1[i] {
            cnt2[(s[l] - b'a') as usize] -= 1;
            l += 1;
        }
        if r >= l {
            res = res.max(r - l + 1);
        }
    }

    (n - res) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(take_characters("aabaaaacaabc".to_string(), 2), 8);
        assert_eq!(take_characters("a".to_string(), 1), -1);
        assert_eq!(take_characters("ccbabcc".to_string(), 1), 4);
    }
}
