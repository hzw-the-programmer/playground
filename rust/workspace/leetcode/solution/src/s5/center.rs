// time: O(n^2)
// space: O(1)
pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let n = s.len();

    let (mut start, mut end) = (0, 0);
    for i in 0..n - 1 {
        let (s, e) = expand_around_center(bytes, i, i);
        if end - start + 1 < e - s + 1 {
            start = s;
            end = e;
        }

        if bytes[i] == bytes[i + 1] {
            let (s, e) = expand_around_center(bytes, i, i + 1);
            if end - start + 1 < e - s + 1 {
                start = s;
                end = e;
            }
        }
    }

    s[start..=end].to_string()
}

fn expand_around_center(s: &[u8], mut left: usize, mut right: usize) -> (usize, usize) {
    while left > 0 && right < s.len() - 1 && s[left - 1] == s[right + 1] {
        left -= 1;
        right += 1;
    }
    (left, right)
}

// pub fn longest_palindrome(s: String) -> String {
//     let n = s.len();
//     if n < 2 {
//         return s;
//     }

//     let (mut start, mut end) = (0, 0);
//     for i in 0..n {
//         let len1 = expand(&s, i, i);
//         let len2 = expand(&s, i, i + 1);
//         let len = len1.max(len2);
//         if end - start + 1 < len {
//             start = i - (len - 1) / 2;
//             end = i + len / 2;
//         }
//     }

//     s[start..=end].to_string()
// }

// fn expand(s: &str, left: usize, right: usize) -> usize {
//     let s = s.as_bytes();
//     let mut left = left as i32;
//     let mut right = right;
//     while left >= 0 && right < s.len() && s[left as usize] == s[right] {
//         left -= 1;
//         right += 1;
//     }
//     (right as i32 - left + 1 - 2) as _
// }
