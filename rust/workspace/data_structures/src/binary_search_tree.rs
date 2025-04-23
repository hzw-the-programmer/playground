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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut bst = BinarySearchTree::new();
        assert!(!bst.contains(&5));

        bst.insert(5);
        assert!(bst.contains(&5));

        bst.insert(3);
        bst.insert(7);
        assert!(bst.contains(&3));
        assert!(bst.contains(&7));

        bst.delete(&5);
        assert!(!bst.contains(&5));
        assert!(bst.contains(&3));
        assert!(bst.contains(&7));
    }

    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        assert!(bst.delete(&5));
        assert!(!bst.contains(&5));
        assert!(bst.contains(&3));
        assert!(bst.contains(&7));
        assert!(bst.contains(&6));
        assert!(bst.contains(&8));

        assert!(bst.delete(&7));
        assert!(bst.contains(&6));
        assert!(bst.contains(&8));
        assert!(!bst.contains(&7));
    }

    #[test]
    fn binary_search_tree() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(4);
        bst.insert(3);
        bst.insert(2);
        bst.insert(1);
        bst.insert(0);
        assert_eq!(
            vec![0, 1, 2, 3, 4, 5].iter().collect::<Vec<_>>(),
            bst.in_order()
        );

        assert!(bst.contains(&5));
        assert!(!bst.contains(&6));
    }
}
