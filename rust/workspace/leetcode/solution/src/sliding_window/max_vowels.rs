pub fn max_vowels(s: &[u8], k: usize) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;

    for i in 0..k {
        if vowels.contains(&char::from(s[i])) {
            count += 1;
        }
    }

    let mut max = count;

    for i in k..s.len() {
        if vowels.contains(&char::from(s[i])) {
            count += 1;
        }
        if vowels.contains(&char::from(s[i - k])) {
            count -= 1;
        }
        max = max.max(count);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, max_vowels(b"abciiidef", 3));
        assert_eq!(2, max_vowels(b"aeiou", 2));
        assert_eq!(2, max_vowels(b"leetcode", 3));
    }
}
