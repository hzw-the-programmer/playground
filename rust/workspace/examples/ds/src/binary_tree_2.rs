pub fn test() {
    let mut root = BinaryTree::Empty;
    root.add(10);
    root.add(8);
    root.add(9);
    root.add(7);
    root.add(11);
    root.add(5);
    root.print();
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: PartialOrd + std::fmt::Debug> BinaryTree<T> {
    fn add(&mut self, data: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                    data,
                }))
            }
            BinaryTree::NonEmpty(b) => {
                if data <= b.data {
                    b.left.add(data);
                } else {
                    b.right.add(data);
                }
            }
        }
    }

    fn print(&self) {
        match self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(b) => {
                b.left.print();
                println!("{:?}", b.data);
                b.right.print();
            }
        }
    }
}

struct TreeNode<T> {
    left: BinaryTree<T>,
    right: BinaryTree<T>,
    data: T,
}
