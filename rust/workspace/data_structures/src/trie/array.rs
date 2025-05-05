const CHARSET_LEN: usize = 26;
fn get_index(c: char) -> usize {
    assert!(c >= 'a' && c <= 'z');
    c as usize - 'a' as usize
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; CHARSET_LEN],
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
            let i = get_index(c);
            if let None = current.children[i] {
                current.children[i] = Some(Box::new(Node::new()));
            }
            current = current.children[i].as_mut().unwrap();
        }
        current.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            let i = get_index(c);
            if let None = current.children[i] {
                return false;
            }
            current = current.children[i].as_ref().unwrap();
        }
        current.is_end
    }

    pub fn starts_with(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            let i = get_index(c);
            if let None = current.children[i] {
                return false;
            }
            current = current.children[i].as_ref().unwrap();
        }
        true
    }
}
