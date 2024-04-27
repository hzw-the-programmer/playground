fn main() {
    let mut bt = BinaryTree::Empty;
    bt.add(10);
    bt.add(1);
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

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
