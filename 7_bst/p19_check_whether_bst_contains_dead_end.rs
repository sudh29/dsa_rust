use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn is_dead_end(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut all_nodes = HashSet::new();
    let mut leaf_nodes = HashSet::new();

    fn store_nodes(
        node: &Option<Rc<RefCell<TreeNode>>>,
        all: &mut HashSet<i32>,
        leaves: &mut HashSet<i32>,
    ) {
        if let Some(n) = node {
            let nb = n.borrow();
            all.insert(nb.val);
            if nb.left.is_none() && nb.right.is_none() {
                leaves.insert(nb.val);
            }
            store_nodes(&nb.left, all, leaves);
            store_nodes(&nb.right, all, leaves);
        }
    }
    store_nodes(root, &mut all_nodes, &mut leaf_nodes);
    all_nodes.insert(0);

    for &leaf in &leaf_nodes {
        if all_nodes.contains(&(leaf - 1)) && all_nodes.contains(&(leaf + 1)) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_end() {
        let tree = TreeNode::from_level_order(&[
            Some(8),
            Some(5),
            Some(9),
            Some(2),
            Some(7),
            None,
            None,
            Some(1),
        ]);
        assert!(is_dead_end(&tree));
    }
}
