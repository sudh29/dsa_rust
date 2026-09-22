use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_sum_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match node {
            Some(n) => {
                let nb = n.borrow();
                if nb.left.is_none() && nb.right.is_none() {
                    return (true, nb.val);
                }
                let (left_ok, left_sum) = check(&nb.left);
                let (right_ok, right_sum) = check(&nb.right);
                let ok = left_ok && right_ok && (nb.val == left_sum + right_sum);
                (ok, nb.val + left_sum + right_sum)
            }
            None => (true, 0),
        }
    }
    check(root).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sum_tree() {
        let tree = TreeNode::from_level_order(&[
            Some(26),
            Some(10),
            Some(3),
            Some(4),
            Some(6),
            None,
            Some(3),
        ]);
        assert!(is_sum_tree(&tree));
    }
}
