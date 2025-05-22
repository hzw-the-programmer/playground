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

    if let Some(max) = max(&node.left) {
        if node.val <= max {
            return false;
        }
    }

    if let Some(min) = min(&node.right) {
        if node.val >= min {
            return false;
        }
    }

    true
}

fn max(root: &Tree) -> Option<i32> {
    if root.is_none() {
        return None;
    }

    let mut node = root.clone().unwrap();
    while node.borrow().right.is_some() {
        let n = node.borrow().right.clone();
        node = n.unwrap();
    }
    let val = node.borrow().val;
    Some(val)
}

fn min(root: &Tree) -> Option<i32> {
    if root.is_none() {
        return None;
    }

    let mut node = root.clone().unwrap();
    while node.borrow().left.is_some() {
        let n = node.borrow().left.clone();
        node = n.unwrap();
    }
    let val = node.borrow().val;
    Some(val)
}
