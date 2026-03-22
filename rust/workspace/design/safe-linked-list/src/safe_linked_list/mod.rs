use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

pub struct SafeLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> SafeLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_back(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));

        match self.tail.take() {
            Some(tail) => tail.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone()),
        }

        self.tail = Some(node);
        self.len += 1;
    }

    pub fn remove(&mut self, value: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node<T>>>> = None;

        while let Some(node) = current {
            let node_ref = node.borrow();

            if node_ref.value == *value {
                match prev {
                    Some(prev_node) => {
                        prev_node.borrow_mut().next = node_ref.next.clone();
                        if self.tail.as_ref() == Some(&node) {
                            self.tail = Some(prev_node);
                        }
                    }
                    None => {
                        self.head = node_ref.next.clone();
                        if self.len == 1 {
                            self.tail = None;
                        }
                    }
                }

                self.len -= 1;
                return true;
            }

            prev = Some(node.clone());
            current = node_ref.next.clone();
        }

        false
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.clone(),
        }
    }
}

impl<T> IntoIterator for SafeLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

struct Iter<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();
        if let Some(node) = &current {
            self.current = node.borrow().next.clone();
        }
        current
    }
}

pub struct IntoIter<T> {
    list: SafeLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list.head.take() {
            Some(node) => {
                self.list.head = node.borrow().next.clone();
                self.list.len -= 1;
                if self.list.len == 0 {
                    self.list.tail = None;
                }
                Some(Rc::into_inner(node).unwrap().into_inner().value)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
