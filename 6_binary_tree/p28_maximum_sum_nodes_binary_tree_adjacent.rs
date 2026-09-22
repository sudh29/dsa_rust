use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_max_sum_non_adjacent(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let (incl_l, excl_l) = dfs(&nb.left);
                let (incl_r, excl_r) = dfs(&nb.right);
                let incl = nb.val + excl_l + excl_r;
                let excl = incl_l.max(excl_l) + incl_r.max(excl_r);
                (incl, excl)
            }
            None => (0, 0),
        }
    }
    let (incl, excl) = dfs(root);
    incl.max(excl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_adj() {
        let tree = TreeNode::from_level_order(&[
            Some(1),
            Some(2),
            Some(3),
            Some(1),
            None,
            Some(4),
            Some(5),
        ]);
        assert_eq!(get_max_sum_non_adjacent(&tree), 11);
    }
}
