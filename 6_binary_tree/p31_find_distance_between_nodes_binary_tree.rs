use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn find_dist(root: &Option<Rc<RefCell<TreeNode>>>, a: i32, b: i32) -> isize {
    fn distance_from_node(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> isize {
        match node {
            Some(n) => {
                let nb = n.borrow();
                if nb.val == target {
                    return 0;
                }
                let left = distance_from_node(&nb.left, target);
                if left >= 0 {
                    return left + 1;
                }
                let right = distance_from_node(&nb.right, target);
                if right >= 0 {
                    return right + 1;
                }
                -1
            }
            None => -1,
        }
    }
    let lca = super::p30_find_lca_binary_tree::lowest_common_ancestor(root, a, b);
    let d1 = distance_from_node(&lca, a);
    let d2 = distance_from_node(&lca, b);
    d1 + d2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_dist() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(find_dist(&tree, 2, 3), 2);
    }
}
