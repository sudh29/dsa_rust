use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn flatten_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    TreeNode::to_inorder(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_bst() {
        let tree = TreeNode::from_level_order(&[
            Some(5),
            Some(3),
            Some(7),
            Some(2),
            Some(4),
            Some(6),
            Some(8),
        ]);
        assert_eq!(flatten_bst(&tree), vec![2, 3, 4, 5, 6, 7, 8]);
    }
}
