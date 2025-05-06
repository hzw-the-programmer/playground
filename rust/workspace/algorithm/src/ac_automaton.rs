const CHARSET_SIZE: usize = 26;

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; CHARSET_SIZE],
    fail: Option<Box<Node>>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
pub struct ACAutomaton {
    root: Box<Node>,
}

impl ACAutomaton {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: &str) {
        // let mut current = &mut self.root; // &mut Box<Node>
        let mut current = &mut *self.root; // &mut Node
        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if current.children[index].is_none() {
                current.children[index] = Some(Box::new(Node::new()));
            }
            current = current.children[index].as_mut().unwrap(); // &mut Box<Node>
        }
        current.is_end = true;
    }

    pub fn build(&mut self) {
        todo!();
    }

    pub fn query(&self, text: &str) -> usize {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_word_query() {
        let mut ac = ACAutomaton::new();
        ac.insert("he");
        ac.build();
        // let text = "he";
        // let count = ac.query(text);
        // assert_eq!(1, count);
    }
}
