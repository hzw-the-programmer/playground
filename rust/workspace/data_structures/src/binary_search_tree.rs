struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: T) {
        let mut current = &mut self.root;
        loop {
            match current {
                None => {
                    *current = Some(Box::new(TreeNode {
                        value,
                        left: None,
                        right: None,
                    }));
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

    fn search(&self, value: T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }

        false
    }

    fn to_vec_recursive(&self, node: &Option<Box<TreeNode<T>>>, v: &mut Vec<T>) {
        if let Some(node) = node.as_ref() {
            self.to_vec_recursive(&node.left, v);
            v.push(node.value.clone());
            self.to_vec_recursive(&node.right, v);
        }
    }

    fn to_vec(&self) -> Vec<T> {
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
        assert_eq!(vec![0, 1, 2, 3, 4, 5], bst.to_vec());

        assert!(bst.search(5));
        assert!(!bst.search(6));
    }
}
