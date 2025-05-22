use super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_inorder_traversal() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
        1,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new(
            2,
            Some(Rc::new(RefCell::new(TreeNode::new(3, None, None)))),
            None,
        )))),
    ))));
    assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
}

#[test]
fn test_inorder_traversal_2() {
    let root = None;
    assert_eq!(inorder_traversal(root), vec![]);
}

#[test]
fn test_inorder_traversal_3() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1, None, None))));
    assert_eq!(inorder_traversal(root), vec![1]);
}
