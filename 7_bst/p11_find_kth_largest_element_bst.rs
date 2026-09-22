use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_largest(root: &Option<Rc<RefCell<TreeNode>>>, k: usize) -> Option<i32> {
    let vals = TreeNode::to_inorder(root);
    if k == 0 || k > vals.len() {
        None
    } else {
        Some(vals[vals.len() - k])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        let tree = TreeNode::from_level_order(&[Some(4), Some(2), Some(9)]);
        assert_eq!(kth_largest(&tree, 1), Some(9));
        assert_eq!(kth_largest(&tree, 2), Some(4));
    }
}
