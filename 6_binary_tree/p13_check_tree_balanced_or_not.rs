use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check_height(node: &Option<Rc<RefCell<TreeNode>>>) -> isize {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let lh = check_height(&nb.left);
                if lh == -1 {
                    return -1;
                }
                let rh = check_height(&nb.right);
                if rh == -1 {
                    return -1;
                }
                if (lh - rh).abs() > 1 {
                    -1
                } else {
                    1 + lh.max(rh)
                }
            }
            None => 0,
        }
    }
    check_height(root) != -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        let balanced = TreeNode::from_level_order(&[Some(3), Some(9), Some(20)]);
        assert!(is_balanced(&balanced));
        let unbalanced = TreeNode::from_level_order(&[Some(1), Some(2), None, Some(3), None]);
        assert!(!is_balanced(&unbalanced));
    }
}
