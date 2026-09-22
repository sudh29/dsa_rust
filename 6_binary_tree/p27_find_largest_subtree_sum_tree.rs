use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn find_largest_subtree_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_sum = i32::MIN;
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let left = dfs(&nb.left, max_sum);
                let right = dfs(&nb.right, max_sum);
                let curr_sum = nb.val + left + right;
                *max_sum = (*max_sum).max(curr_sum);
                curr_sum
            }
            None => 0,
        }
    }
    dfs(root, &mut max_sum);
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_subtree() {
        let tree = TreeNode::from_level_order(&[
            Some(1),
            Some(-2),
            Some(3),
            Some(4),
            Some(5),
            Some(-6),
            Some(2),
        ]);
        assert_eq!(find_largest_subtree_sum(&tree), 7);
    }
}
