// 94. Binary Tree Inorder Traversal

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

pub struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

impl TreeNode {
    fn new(val: i32, left: Tree, right: Tree) -> Self {
        Self { val, left, right }
    }
}

// mod recursive;
// pub use recursive::inorder_traversal;

mod iterative;
pub use iterative::inorder_traversal;

#[cfg(test)]
mod tests;
