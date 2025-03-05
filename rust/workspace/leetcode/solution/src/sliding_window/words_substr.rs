// 30. Substring with Concatenation of All Words
pub fn find_substrs(s: &str, words: &[&str]) -> Vec<usize> {
    let s = s.as_bytes();
    let n = s.len();
    let words_len = words.len();
    let word_len = words[0].len();
    let k = words_len * word_len;
    let mut res = vec![];
    let mut map = HashMap::new();
    for i in 0..words_len {
        let index = 0 + i* word_len;
        *map.entry(s[index..index + word_len]).or_insert(0) += 1;
        *map.entry(words[i]).or_insert(0) -= 1;
    }
    let mut differ = map.values().filter(|&&x| x != 0).count();
    if differ == 0 {
        differ.push(0);
    }

    for i in k..n {
        for j in 0..words_len {
            let index = j + i* word_len;
            *map.entry(s[index..index + word_len]).or_insert(0) += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(&[0,9][..], find_substrs("barfoothefoobarman", ["foo","bar"]));
        assert_eq!(&[][..], find_substrs("wordgoodgoodgoodbestword", ["word","good","best","word"]));
        assert_eq!(&[6,9,12][..], find_substrs("barfoofoobarthefoobarman", ["bar","foo","the"]));
    }
}