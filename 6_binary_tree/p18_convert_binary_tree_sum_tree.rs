use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn to_sum_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(n) => {
            let mut nb = n.borrow_mut();
            let old_val = nb.val;
            let left_sum = to_sum_tree(&nb.left);
            let right_sum = to_sum_tree(&nb.right);
            nb.val = left_sum + right_sum;
            old_val + nb.val
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_sum_tree() {
        let tree = TreeNode::from_level_order(&[
            Some(10),
            Some(-2),
            Some(6),
            Some(8),
            Some(-4),
            Some(7),
            Some(5),
        ]);
        to_sum_tree(&tree);
        assert_eq!(tree.as_ref().unwrap().borrow().val, 20);
    }
}
