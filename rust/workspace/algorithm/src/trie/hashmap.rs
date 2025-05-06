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
