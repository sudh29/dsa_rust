use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn max_element_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    let vals = TreeNode::to_inorder(root);
    vals.into_iter().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_elem() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(12), Some(3)]);
        assert_eq!(max_element_tree(&tree), Some(12));
    }
}
