use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
    match root {
        Some(n) => {
            let nb = n.borrow();
            nb.val == target || find_node(&nb.left, target) || find_node(&nb.right, target)
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_node() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert!(find_node(&tree, 3));
        assert!(!find_node(&tree, 99));
    }
}
