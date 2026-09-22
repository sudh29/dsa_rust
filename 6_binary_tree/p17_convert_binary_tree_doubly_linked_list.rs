use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn b_to_dll(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    TreeNode::to_inorder(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_to_dll() {
        let tree = TreeNode::from_level_order(&[Some(10), Some(20), Some(30), Some(40), Some(60)]);
        assert_eq!(b_to_dll(&tree), vec![40, 20, 60, 10, 30]);
    }
}
