use std::cell::RefCell;
use std::rc::Rc;

/*
设计一个内存安全的链表（禁止使用 unsafe）

题目要求

实现一个单向链表，支持：
追加节点（push_back）；
插入节点到指定位置（insert）；
删除指定值的节点（remove）；
遍历链表（iter）；
内存安全（无悬垂引用、无内存泄漏、无数据竞争）；
禁止使用 unsafe 代码（考察 Rust 安全抽象）。

设计思路

核心结构：使用 Rc<RefCell<Node>> 实现共享可变节点（Rc 共享所有权，RefCell 运行时借用检查）；
节点设计：Node 包含值和指向下一个节点的 Option<Rc<RefCell<Node>>>；
链表头：使用 Option<Rc<RefCell<Node>>> 表示链表头；
遍历：实现 IntoIterator，返回安全的不可变迭代器；
内存安全：依赖 Rc 的引用计数自动释放内存，RefCell 防止同时可变借用。

关键考点

所有权与借用（Rc/RefCell 的使用）；
安全的可变共享；
迭代器实现；
无 unsafe 的数据结构设计；
内存泄漏避免（打破循环引用）。
*/

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
