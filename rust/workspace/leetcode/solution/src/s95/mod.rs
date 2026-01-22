// 95. Unique Binary Search Trees II

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

mod backtrack;
pub use backtrack::generate_trees;

#[cfg(test)]
mod tests;
