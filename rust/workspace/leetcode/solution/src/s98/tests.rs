use super::*;

#[test]
fn test_is_valid_bst() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    assert_eq!(is_valid_bst(root), true);
}

#[test]
fn test_is_valid_bst_2() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(is_valid_bst(root), false);
}

#[test]
fn test_is_valid_bst_3() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
        5,
        Some(Rc::new(RefCell::new(TreeNode::new(4, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new(
            6,
            Some(Rc::new(RefCell::new(TreeNode::new(3, None, None)))),
            Some(Rc::new(RefCell::new(TreeNode::new(7, None, None)))),
        )))),
    ))));
    assert_eq!(is_valid_bst(root), false);
}
