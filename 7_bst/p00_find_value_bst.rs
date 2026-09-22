use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn search_bst(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
    match root {
        Some(node) => {
            let nb = node.borrow();
            if nb.val == val {
                true
            } else if val < nb.val {
                search_bst(&nb.left, val)
            } else {
                search_bst(&nb.right, val)
            }
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_bst() {
        let tree = TreeNode::from_level_order(&[Some(4), Some(2), Some(7), Some(1), Some(3)]);
        assert!(search_bst(&tree, 2));
        assert!(!search_bst(&tree, 5));
    }
}
