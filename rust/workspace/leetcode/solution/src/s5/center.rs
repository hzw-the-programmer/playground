// time: O(n^2)
// space: O(1)
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    if n < 2 {
        return s;
    }

    let (mut start, mut end) = (0, 0);
    for i in 0..n {
        let len1 = expand(&s, i, i);
        let len2 = expand(&s, i, i + 1);
        let len = len1.max(len2);
        if end - start + 1 < len {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }

    s[start..=end].to_string()
}

fn expand(s: &str, left: usize, right: usize) -> usize {
    let s = s.as_bytes();
    let mut left = left as i32;
    let mut right = right;
    while left >= 0 && right < s.len() && s[left as usize] == s[right] {
        left -= 1;
        right += 1;
    }
    (right as i32 - left + 1 - 2) as _
}
