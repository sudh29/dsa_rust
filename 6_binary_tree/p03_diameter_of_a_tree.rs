use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, max_d: &mut usize) -> usize {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let lh = helper(&nb.left, max_d);
                let rh = helper(&nb.right, max_d);
                *max_d = (*max_d).max(lh + rh);
                1 + lh.max(rh)
            }
            None => 0,
        }
    }
    let mut max_d = 0;
    helper(root, &mut max_d);
    max_d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diameter() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter(&tree), 3);
    }
}
