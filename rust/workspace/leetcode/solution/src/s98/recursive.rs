use super::*;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_recursive(&root)
}

fn is_valid_bst_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut res = true;
    if let Some(node) = root {
        let node = node.borrow();

        if let Some(left) = &node.left {
            res = res
                && if node.val > left.borrow().val {
                    is_valid_bst_recursive(&node.left)
                } else {
                    false
                }
        }

        if !res {
            return false;
        }

        if let Some(right) = &node.right {
            res = res
                && if node.val < right.borrow().val {
                    is_valid_bst_recursive(&node.right)
                } else {
                    false
                }
        }
    }
    res
}
