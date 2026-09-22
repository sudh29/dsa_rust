use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn bst_from_preorder(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    let root = TreeNode::new_rc(preorder[0]);
    let split = preorder[1..]
        .iter()
        .position(|&x| x > preorder[0])
        .map(|p| p + 1)
        .unwrap_or(preorder.len());

    root.borrow_mut().left = bst_from_preorder(&preorder[1..split]);
    root.borrow_mut().right = bst_from_preorder(&preorder[split..]);
    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_preorder() {
        let pre = [8, 5, 1, 7, 10, 12];
        let root = bst_from_preorder(&pre);
        assert_eq!(TreeNode::to_inorder(&root), vec![1, 5, 7, 8, 10, 12]);
    }
}
