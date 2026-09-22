use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_of_longest_path(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (usize, i32) {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let (lh, ls) = dfs(&nb.left);
                let (rh, rs) = dfs(&nb.right);
                if lh > rh {
                    (lh + 1, ls + nb.val)
                } else if rh > lh {
                    (rh + 1, rs + nb.val)
                } else {
                    (lh + 1, ls.max(rs) + nb.val)
                }
            }
            None => (0, 0),
        }
    }
    dfs(root).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_path_sum() {
        let tree = TreeNode::from_level_order(&[
            Some(4),
            Some(2),
            Some(5),
            Some(7),
            Some(1),
            Some(2),
            Some(3),
        ]);
        assert_eq!(sum_of_longest_path(&tree), 13);
    }
}
