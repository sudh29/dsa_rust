use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn balance_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let vals = TreeNode::to_inorder(root);
    fn build(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }
        let mid = vals.len() / 2;
        let root = TreeNode::new_rc(vals[mid]);
        root.borrow_mut().left = build(&vals[..mid]);
        root.borrow_mut().right = build(&vals[mid + 1..]);
        Some(root)
    }
    build(&vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_bst() {
        let tree = TreeNode::from_level_order(&[Some(1), None, Some(2), None, Some(3)]);
        let balanced = balance_bst(&tree);
        assert_eq!(TreeNode::to_inorder(&balanced), vec![1, 2, 3]);
    }
}
