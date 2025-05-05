use crate::trie::Trie;

// https://www.geeksforgeeks.org/word-break-problem-trie-solution/
// Time Complexity: O(n*m) where n is the length of the string and m is the length of the longest word in the dictionary.
// Auxiliary Space: O(n)
pub fn word_break(s: &str, dictionary: &[&str]) -> bool {
    let mut trie = Trie::new();
    dictionary.iter().for_each(|w| trie.insert(w));

    word_break_recursive(s, &trie)
}

fn word_break_recursive(s: &str, trie: &Trie) -> bool {
    let n = s.len();
    if n == 0 {
        true
    } else {
        for i in 1..=n {
            if trie.starts_with(&s[..i]) && word_break_recursive(&s[i..], trie) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        let dictionary = [
            "mobile", "samsung", "sam", "sung", "ma\n", "mango", "icecream", "and", "go", "i",
            "like", "ice", "cream",
        ];
        assert!(word_break("ilikesamsung", &dictionary));
        assert!(word_break("iiiiiiii", &dictionary));
        assert!(word_break("", &dictionary));
        assert!(word_break("ilikelikeimangoiii", &dictionary));
        assert!(word_break("samsungandmango", &dictionary));
        assert!(!word_break("samsungandmangok", &dictionary));

        let dictionary = ["leet", "code"];
        assert!(word_break("leetcode", &dictionary));

        let dictionary = ["apple", "pen"];
        assert!(word_break("applepenapple", &dictionary));

        let dictionary = ["cats", "pen", "sand", "and", "cat"];
        assert!(!word_break("catsandog", &dictionary));

        let dictionary = ["aaa", "aaaa"];
        assert!(word_break("aaaaa", &dictionary));

        let s = "a";
        assert!(word_break(s, &["a"]));
        assert!(!word_break(s, &["b"]));

        let s = "";
        assert!(word_break(s, &[""]));
        assert!(word_break(s, &["a"]));
    }
}
