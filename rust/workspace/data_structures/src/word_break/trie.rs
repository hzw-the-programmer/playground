/*
  https://www.geeksforgeeks.org/word-break-problem-trie-solution/
  Time Complexity: O(n*m) where n is the length of the string and m is the length of the longest word in the dictionary.
  Auxiliary Space: O(n)
*/

use crate::trie::Trie;

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
            if trie.search(&s[..i]) && word_break_recursive(&s[i..], trie) {
                return true;
            }
        }

        false
    }
}
