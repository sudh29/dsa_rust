use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn reverse_inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vals = TreeNode::to_inorder(root);
    vals.reverse();
    vals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_inorder() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(reverse_inorder(&tree), vec![3, 1, 2]);
    }
}
