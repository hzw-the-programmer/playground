use std::collections::HashMap;

#[derive(Default)]
struct Node {
    children: HashMap<char, Node>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for c in word.chars() {
            current = current.children.entry(c).or_insert(Node::new());
        }

        current.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;

        for c in word.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }

        current.is_end
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;

        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(trie.search("hello"));
        assert!(!trie.search("he"));
        assert!(!trie.search("world"));
    }

    #[test]
    fn test_starts_with() {
        let mut trie = Trie::new();
        trie.insert("apple");
        assert!(trie.starts_with("app"));
        assert!(!trie.starts_with("ban"));
    }

    #[test]
    fn test_empty_trie() {
        let trie = Trie::new();
        assert!(!trie.search("anyword"));
        assert!(!trie.starts_with("anyprefix"));
    }

    #[test]
    fn test_insert_multible_words() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("dog");
        trie.insert("cattle");
        assert!(trie.search("cat"));
        assert!(trie.search("dog"));
        assert!(trie.search("cattle"));
        assert!(!trie.search("rat"));
        assert!(trie.starts_with("ca"));
    }
}
