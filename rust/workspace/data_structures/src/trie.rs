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
            if let Some(node) = current.children.get(&c) {
                current = node;
            } else {
                return false;
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
        assert!(!trie.search("world"));
    }
}
