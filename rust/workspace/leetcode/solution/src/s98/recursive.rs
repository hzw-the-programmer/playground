use super::*;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_recursive(&root)
}

fn is_valid_bst_recursive(root: &Tree) -> bool {
    if root.is_none() {
        return true;
    }

    let node = root.as_ref().unwrap().borrow();
    if !is_valid_bst_recursive(&node.left) || !is_valid_bst_recursive(&node.right) {
        return false;
    }

    if let Some(left) = &node.left {
        if node.val <= max(left.clone()) {
            return false;
        }
    }

    if let Some(right) = &node.right {
        if node.val >= min(right.clone()) {
            return false;
        }
    }

    true
}

fn max(mut rc: Rc<RefCell<TreeNode>>) -> i32 {
    while rc.borrow().right.is_some() {
        let node = rc.borrow().right.clone();
        rc = node.unwrap();
    }

    rc.borrow().val
}

fn min(mut rc: Rc<RefCell<TreeNode>>) -> i32 {
    while rc.borrow().left.is_some() {
        let node = rc.borrow().left.clone();
        rc = node.unwrap();
    }

    rc.borrow().val
}
