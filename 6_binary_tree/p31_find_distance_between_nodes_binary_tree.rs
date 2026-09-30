use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Computes the distance (number of edges) between two nodes with values `a` and `b` in a binary tree.
///
/// # Complexity
/// - Time Complexity: $O(N)$
/// - Space Complexity: $O(H)$ where $H$ is the tree height
///
/// # Examples
/// ```
/// use dsa_rust::binary_tree::p31_find_distance_between_nodes_binary_tree::find_dist;
/// use dsa_rust::common::tree_node::TreeNode;
///
/// let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
/// assert_eq!(find_dist(&tree, 2, 3), Some(2));
/// ```
pub fn find_dist(root: &Option<Rc<RefCell<TreeNode>>>, a: i32, b: i32) -> Option<usize> {
    fn distance_from_node(node: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<usize> {
        let n = node.as_ref()?;
        let nb = n.borrow();
        if nb.val == target {
            return Some(0);
        }
        if let Some(left) = distance_from_node(&nb.left, target) {
            return Some(left + 1);
        }
        if let Some(right) = distance_from_node(&nb.right, target) {
            return Some(right + 1);
        }
        None
    }

    let lca = super::p30_find_lca_binary_tree::lowest_common_ancestor(root, a, b)?;
    let d1 = distance_from_node(&Some(Rc::clone(&lca)), a)?;
    let d2 = distance_from_node(&Some(lca), b)?;
    Some(d1 + d2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_dist() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(find_dist(&tree, 2, 3), Some(2));
        assert_eq!(find_dist(&tree, 2, 99), None);
        assert_eq!(find_dist(&None, 1, 2), None);
    }
}
