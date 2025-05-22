use super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_inorder_traversal() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // the trait `DerefMut` is not implemented for `Rc<RefCell<s94::TreeNode>>`
    // let mut i: i32 = root.as_deref_mut();

    // expected `i32`, found `Option<&RefCell<TreeNode>>`
    // let mut i: i32 = root.as_deref();

    // expected `i32`, found `Option<&mut Rc<RefCell<TreeNode>>>`
    // let mut i: i32 = root.as_mut();

    // expected `i32`, found `Option<&Rc<RefCell<TreeNode>>>`
    // let mut i: i32 = root.as_ref();

    {
        let mut current = root.as_ref().unwrap().borrow_mut();
        current.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut current = current.right.as_ref().unwrap().borrow_mut();
        current.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    }

    assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
}
