use super::*;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    inorder_traversal_recursive(&root, &mut res);
    res
}

fn inorder_traversal_recursive(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder_traversal_recursive(&node.left, res);
        res.push(node.val);
        inorder_traversal_recursive(&node.right, res);
    }
}
