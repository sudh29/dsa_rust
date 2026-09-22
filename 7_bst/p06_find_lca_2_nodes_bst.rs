use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lca_bst(root: &Option<Rc<RefCell<TreeNode>>>, n1: i32, n2: i32) -> Option<i32> {
    let mut curr = root.as_ref().map(Rc::clone);
    while let Some(node) = curr {
        let val = node.borrow().val;
        if val > n1 && val > n2 {
            curr = node.borrow().left.as_ref().map(Rc::clone);
        } else if val < n1 && val < n2 {
            curr = node.borrow().right.as_ref().map(Rc::clone);
        } else {
            return Some(val);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lca_bst() {
        let tree = TreeNode::from_level_order(&[
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
        ]);
        assert_eq!(lca_bst(&tree, 2, 8), Some(6));
        assert_eq!(lca_bst(&tree, 2, 4), Some(2));
    }
}
