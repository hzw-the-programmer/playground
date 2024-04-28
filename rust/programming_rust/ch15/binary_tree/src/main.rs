fn main() {
    let mut bt = BinaryTree::Empty;
    bt.add(10);
    bt.add(1);
}

#[derive(PartialEq, Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(PartialEq, Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: PartialOrd> BinaryTree<T> {
    fn add(&mut self, element: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if element < node.element {
                    node.left.add(element)
                } else {
                    node.right.add(element)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: 1,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }));
        let b = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: 3,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }));
        let c = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: 2,
            left: a,
            right: b,
        }));

        let mut bt = BinaryTree::Empty;
        bt.add(2);
        bt.add(3);
        bt.add(1);

        assert_eq!(bt, c);
    }
}
