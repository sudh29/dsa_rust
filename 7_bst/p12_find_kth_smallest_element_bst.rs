use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_smallest(root: &Option<Rc<RefCell<TreeNode>>>, k: usize) -> Option<i32> {
    let vals = TreeNode::to_inorder(root);
    if k == 0 || k > vals.len() {
        None
    } else {
        Some(vals[k - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        let tree = TreeNode::from_level_order(&[Some(4), Some(2), Some(9)]);
        assert_eq!(kth_smallest(&tree, 1), Some(2));
        assert_eq!(kth_smallest(&tree, 2), Some(4));
    }
}
