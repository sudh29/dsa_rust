use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn check_leaf_level(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut levels = HashSet::new();
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, lvl: usize, levels: &mut HashSet<usize>) {
        if let Some(n) = node {
            let nb = n.borrow();
            if nb.left.is_none() && nb.right.is_none() {
                levels.insert(lvl);
            } else {
                dfs(&nb.left, lvl + 1, levels);
                dfs(&nb.right, lvl + 1, levels);
            }
        }
    }
    dfs(root, 0, &mut levels);
    levels.len() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_same_level() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert!(check_leaf_level(&tree));
        let tree2 = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4)]);
        assert!(!check_leaf_level(&tree2));
    }
}
