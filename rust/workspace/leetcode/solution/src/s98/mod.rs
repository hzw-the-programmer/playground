// 98. Validate Binary Search Tree

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
// pub use recursive::is_valid_bst;

mod recursive2;
pub use recursive2::is_valid_bst;

#[cfg(test)]
mod tests;
