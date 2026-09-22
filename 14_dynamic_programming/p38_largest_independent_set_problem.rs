use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn liss(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (usize, usize) {
        // returns (incl, excl)
        match node {
            None => (0, 0),
            Some(n) => {
                let borrowed = n.borrow();
                let (l_incl, l_excl) = dfs(&borrowed.left);
                let (r_incl, r_excl) = dfs(&borrowed.right);
                let incl = 1 + l_excl + r_excl;
                let excl = l_incl.max(l_excl) + r_incl.max(r_excl);
                (incl, excl)
            }
        }
    }
    let (incl, excl) = dfs(root);
    incl.max(excl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liss() {
        let root = TreeNode::from_level_order(&[
            Some(10),
            Some(20),
            Some(30),
            Some(40),
            Some(50),
            None,
            Some(60),
        ]);
        assert_eq!(liss(&root), 4);
    }
}
