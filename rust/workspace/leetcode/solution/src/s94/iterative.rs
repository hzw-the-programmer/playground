use super::*;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();

    let mut stack = vec![];
    let mut node = root;
    while !node.is_none() || !stack.is_empty() {
        while let Some(rc) = node {
            stack.push(rc.clone());
            node = rc.borrow().left.clone();
        }

        if let Some(rc) = stack.pop() {
            res.push(rc.borrow().val);
            node = rc.borrow().right.clone();
        }
    }

    res
}
