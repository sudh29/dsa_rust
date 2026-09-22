use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let vals = TreeNode::to_inorder(root);
    for i in 1..vals.len() {
        if vals[i] <= vals[i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bst() {
        let bst = TreeNode::from_level_order(&[Some(2), Some(1), Some(3)]);
        assert!(is_bst(&bst));
        let not_bst =
            TreeNode::from_level_order(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert!(!is_bst(&not_bst));
    }
}
