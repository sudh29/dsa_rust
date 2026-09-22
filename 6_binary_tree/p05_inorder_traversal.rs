use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    TreeNode::to_inorder(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(inorder(&tree), vec![2, 1, 3]);
    }
}
