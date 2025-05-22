use super::*;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut inorder = i32::MIN;
    let mut tree = root;
    let mut stack = vec![];
    while tree.is_some() || !stack.is_empty() {
        while let Some(node) = tree {
            stack.push(node.clone());
            tree = node.borrow().left.clone();
        }
        if let Some(node) = stack.pop() {
            let node = node.borrow();
            if node.val <= inorder {
                return false;
            }
            inorder = node.val;
            tree = node.right.clone();
        }
    }
    true
}
