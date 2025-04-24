use std::collections::HashMap;

struct Node {
    children: HashMap<char, Node>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new() }
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
}
