use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn find_median_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> f64 {
    let vals = TreeNode::to_inorder(root);
    let n = vals.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        vals[n / 2] as f64
    } else {
        (vals[n / 2] + vals[n / 2 - 1]) as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_bst() {
        let tree = TreeNode::from_level_order(&[
            Some(6),
            Some(3),
            Some(8),
            Some(1),
            Some(4),
            Some(7),
            Some(9),
        ]);
        assert_eq!(find_median_bst(&tree), 6.0);
    }
}
