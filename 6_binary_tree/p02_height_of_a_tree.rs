use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    match root {
        Some(node) => {
            let n = node.borrow();
            1 + height(&n.left).max(height(&n.right))
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(height(&tree), 3);
        assert_eq!(height(&None), 0);
    }
}
