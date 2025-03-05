use std::collections::HashMap;

// 30. Substring with Concatenation of All Words
pub fn find_substr(s: &str, words: &[&str]) -> Vec<usize> {
    let s = s.as_bytes();
    let str_len = s.len();
    let num_of_words = words.len();
    let word_len = words[0].len();
    let substr_len = num_of_words * word_len;
    let mut res = vec![];

    let mut map = HashMap::new();
    for w in words {
        *map.entry(w.as_bytes()).or_insert(0) -= 1;
    }

    for i in 0..num_of_words {
        let (mut left, mut right) = (i, i);

        while right + word_len <= str_len {
            let word = &s[right..right + word_len];
            right += word_len;
            let has = map.contains_key(word);
            *map.entry(word).or_insert(0) += 1;
            while *map.get(word).unwrap() > 0 {
                let removed = &s[left..left + word_len];
                left += word_len;
                *map.get_mut(removed).unwrap() -= 1;
            }
            if !has {
                map.remove(word);
            }
            if right - left == substr_len {
                res.push(left);
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
        assert_eq!(
            &[0, 9][..],
            find_substr("barfoothefoobarman", &["foo", "bar"])
        );
        assert_eq!(
            &[0; 0][..],
            find_substr(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "word"]
            )
        );
        assert_eq!(
            &[6, 9, 12][..],
            find_substr("barfoofoobarthefoobarman", &["bar", "foo", "the"])
        );
    }
}
