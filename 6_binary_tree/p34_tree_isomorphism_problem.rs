use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_isomorphic(
    n1: &Option<Rc<RefCell<TreeNode>>>,
    n2: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (n1, n2) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let ab = a.borrow();
            let bb = b.borrow();
            if ab.val != bb.val {
                return false;
            }
            (is_isomorphic(&ab.left, &bb.left) && is_isomorphic(&ab.right, &bb.right))
                || (is_isomorphic(&ab.left, &bb.right) && is_isomorphic(&ab.right, &bb.left))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isomorphic() {
        let t1 = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        let t2 = TreeNode::from_level_order(&[Some(1), Some(3), Some(2)]);
        assert!(is_isomorphic(&t1, &t2));
    }
}
