struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_front(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    pub fn push_back(&mut self, value: T) {
        let node = Box::new(Node { value, next: None });

        match self.head.as_mut() {
            Some(mut current) => {
                while let Some(ref mut node) = current.next {
                    current = node;
                }
                current.next = Some(node);
            }
            None => self.head = Some(node),
        }

        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len < 2 {
            return self.pop_front();
        }

        let mut current = self.head.as_mut().unwrap();

        while let Some(node) = current.next.take() {
            if node.next.is_none() {
                self.len -= 1;
                return Some(node.value);
            }
            current.next = Some(node);
            current = current.next.as_mut().unwrap();
        }

        unreachable!()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        let mut current = self.head.as_ref();

        for _ in 0..index {
            current = current.unwrap().next.as_ref();
        }

        current.map(|node| &node.value)
    }
}

#[cfg(test)]
mod tests;
