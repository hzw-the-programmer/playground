// 1456. Maximum Number of Vowels in a Substring of Given Length

pub fn max_vowels(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let k = k as usize;

    let vowels = [b'a', b'e', b'i', b'o', b'u'];
    let mut max = 0;
    let mut ans = 0;

    for (i, &b) in s.iter().enumerate() {
        // 1. enter window
        if vowels.contains(&b) {
            max += 1;
        }

        // 2. less than window size
        if i < k - 1 {
            continue;
        }

        // 3. update
        ans = max.max(ans);

        // 4. leave window
        if vowels.contains(&s[i + 1 - k]) {
            max -= 1;
        }
    }

    ans
}

pub fn max_vowels_v2(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let k = k as usize;

    let is_vowel = |b| b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u';
    
    let mut max = s[0..k].iter().filter(|&&b| is_vowel(b)).count() as i32;
    let mut ans = max;

    for i in k..s.len() {
        if is_vowel(s[i]) {
            max += 1;
        }
        
        if is_vowel(s[i - k]) {
            max -= 1;
        }
        
        ans = ans.max(max);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(f: fn(String, i32) -> i32) {
        assert_eq!(3, f("abciiidef".to_string(), 3));
        assert_eq!(2, f("aeiou".to_string(), 2));
        assert_eq!(2, f("leetcode".to_string(), 3));
        assert_eq!(0, f("rhythms".to_string(), 4));
        assert_eq!(1, f("tryhard".to_string(), 4));
    }

    #[test]
    fn test_max_vowels() {
        test(max_vowels);
    }

    #[test]
    fn test_max_vowels_v2() {
        test(max_vowels_v2);
    }
}
