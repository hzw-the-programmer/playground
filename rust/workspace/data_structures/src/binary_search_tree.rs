struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T:Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let mut current = &mut self.root;
        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node::new(value)));
                    break;
                }
                Some(node) => {
                    if value < node.value {
                        current = &mut node.left;
                    } else if value > node.value {
                        current = &mut node.right;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if *value == node.value {
                return true;
            } else if *value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }

    fn to_vec_recursive<'a>(&self, node: &'a Option<Box<Node<T>>>, v: &mut Vec<&'a T>) {
        if let Some(node) = node {
            self.to_vec_recursive(&node.left, v);
            v.push(&node.value);
            self.to_vec_recursive(&node.right, v);
        }
    }

    pub fn to_vec(&self) -> Vec<&T> {
        let mut v = vec![];
        self.to_vec_recursive(&self.root, &mut v);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_tree() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(4);
        bst.insert(3);
        bst.insert(2);
        bst.insert(1);
        bst.insert(0);
        assert_eq!(vec![0, 1, 2, 3, 4, 5].iter().collect::<Vec<_>>(), bst.to_vec());

        assert!(bst.contains(&5));
        assert!(!bst.contains(&6));
    }
}
