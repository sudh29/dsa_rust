use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: &Option<Rc<RefCell<TreeNode>>>,
    n1: i32,
    n2: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let val = node.borrow().val;
            if val == n1 || val == n2 {
                return Some(Rc::clone(node));
            }
            let left_lca = lowest_common_ancestor(&node.borrow().left, n1, n2);
            let right_lca = lowest_common_ancestor(&node.borrow().right, n1, n2);
            if left_lca.is_some() && right_lca.is_some() {
                Some(Rc::clone(node))
            } else {
                left_lca.or(right_lca)
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lca() {
        let tree = TreeNode::from_level_order(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let lca = lowest_common_ancestor(&tree, 4, 5);
        assert_eq!(lca.unwrap().borrow().val, 2);
    }
}
