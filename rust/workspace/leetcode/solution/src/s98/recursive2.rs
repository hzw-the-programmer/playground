use super::*;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_recursive(root, i32::MIN, i32::MAX)
}

fn is_valid_bst_recursive(root: Tree, lower: i32, upper: i32) -> bool {
    if root.is_none() {
        return true;
    }

    let node = root.as_ref().unwrap().borrow();
    if node.val <= lower || node.val >= upper {
        return false;
    }

    is_valid_bst_recursive(node.left.clone(), lower, node.val)
        && is_valid_bst_recursive(node.right.clone(), node.val, upper)
}
