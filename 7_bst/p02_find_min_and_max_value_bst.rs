use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn find_min_max(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32)> {
    let vals = TreeNode::to_inorder(root);
    if vals.is_empty() {
        None
    } else {
        Some((vals[0], *vals.last().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_bst() {
        let tree = TreeNode::from_level_order(&[Some(5), Some(3), Some(8), Some(1), Some(4)]);
        assert_eq!(find_min_max(&tree), Some((1, 8)));
    }
}
