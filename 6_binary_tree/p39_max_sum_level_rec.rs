use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn max_level_sum_recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut level_sums = Vec::new();
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, lvl: usize, sums: &mut Vec<i32>) {
        if let Some(n) = node {
            let nb = n.borrow();
            if lvl >= sums.len() {
                sums.push(nb.val);
            } else {
                sums[lvl] += nb.val;
            }
            dfs(&nb.left, lvl + 1, sums);
            dfs(&nb.right, lvl + 1, sums);
        }
    }
    dfs(root, 0, &mut level_sums);
    level_sums.into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_level_sum_rec() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(7), Some(0), Some(7), Some(-8)]);
        assert_eq!(max_level_sum_recursive(&tree), 7);
    }
}
