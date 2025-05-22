use super::*;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v = Vec::new();
    inorder_traversal_recursive(root.as_ref(), &mut v);
    v
}

fn inorder_traversal_recursive(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }

    let node = root.unwrap().borrow();
    inorder_traversal_recursive(node.left.as_ref(), res);
    res.push(node.val);
    inorder_traversal_recursive(node.right.as_ref(), res);
}
