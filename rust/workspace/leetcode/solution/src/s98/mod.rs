// 98. Validate Binary Search Tree

use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// impl TreeNode {
//     fn new(val: i32) -> Self {
//         Self {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

mod recursive;
pub use recursive::is_valid_bst;

#[cfg(test)]
mod tests;
