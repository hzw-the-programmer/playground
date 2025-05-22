struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(mut root: &mut Option<Box<Node<T>>>, value: T) {
        while let Some(node) = root {
            if value < node.value {
                root = &mut node.left;
            } else if value > node.value {
                root = &mut node.right;
            } else {
                return;
            }
        }

        *root = Some(Box::new(Node::new(value)));
    }

    fn contains(mut root: &Option<Box<Node<T>>>, value: &T) -> bool {
        while let Some(node) = root {
            if value < &node.value {
                root = &node.left;
            } else if value > &node.value {
                root = &node.right;
            } else {
                return true;
            }
        }

        false
    }

    fn delete(mut root: &mut Option<Box<Node<T>>>, value: &T) -> Result<(), ()> {
        while let Some(node) = root {
            if *value < node.value {
                root = &mut root.as_mut().unwrap().left;
            } else if *value > node.value {
                root = &mut root.as_mut().unwrap().right;
            } else {
                match (node.left.as_ref(), node.right.as_ref()) {
                    (None, None) => *root = None,
                    (Some(_), None) => *root = node.left.take(),
                    (None, Some(_)) => *root = node.right.take(),
                    (Some(_), Some(_)) => node.value = Node::delete_min(&mut node.right).unwrap(),
                }
                return Ok(());
            }
        }

        Err(())
    }

    fn delete_min(mut root: &mut Option<Box<Node<T>>>) -> Option<T> {
        if root.is_some() {
            while root.as_ref().unwrap().left.is_some() {
                root = &mut root.as_mut().unwrap().left;
            }

            let node = root.take().unwrap();
            *root = node.right;
            return Some(node.value);
        }

        None
    }
}

pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        Node::insert(&mut self.root, value);
    }

    pub fn contains(&self, value: &T) -> bool {
        Node::contains(&self.root, value)
    }

    pub fn delete(&mut self, value: &T) -> bool {
        Node::delete(&mut self.root, value).is_ok()
    }

    pub fn in_order(&self) -> Vec<&T> {
        let mut v = vec![];
        Self::in_order_traversal(&self.root, &mut v);
        v
    }

    fn in_order_traversal<'a>(node: &'a Option<Box<Node<T>>>, v: &mut Vec<&'a T>) {
        if let Some(n) = node {
            Self::in_order_traversal(&n.left, v);
            v.push(&n.value);
            Self::in_order_traversal(&n.right, v);
        }
    }

    pub fn inorder_iterative(&self) -> Vec<&T> {
        let mut res = vec![];
        let mut tree = &self.root;
        let mut stack = vec![];
        while tree.is_some() || !stack.is_empty() {
            while let Some(node) = tree {
                stack.push(node);
                tree = &node.left;
            }

            if let Some(node) = stack.pop() {
                res.push(&node.value);
                tree = &node.right;
            }
        }
        res
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(&self.root)
    }
}

pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    fn new(root: &'a Option<Box<Node<T>>>) -> Self {
        let mut i = Iter { stack: Vec::new() };
        i.load(root);
        i
    }

    fn load(&mut self, root: &'a Option<Box<Node<T>>>) {
        let mut current = root;
        while let Some(node) = current {
            self.stack.push(node);
            current = &node.left;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            self.load(&node.right);
            return Some(&node.value);
        }

        None
    }
}
