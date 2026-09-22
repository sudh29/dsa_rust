use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn predecessor_successor(
    root: &Option<Rc<RefCell<TreeNode>>>,
    key: i32,
) -> (Option<i32>, Option<i32>) {
    let vals = TreeNode::to_inorder(root);
    let mut pred = None;
    let mut succ = None;

    for &x in &vals {
        if x < key {
            pred = Some(x);
        } else if x > key && succ.is_none() {
            succ = Some(x);
        }
    }
    (pred, succ)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pred_succ() {
        let tree = TreeNode::from_level_order(&[
            Some(50),
            Some(30),
            Some(70),
            Some(20),
            Some(40),
            Some(60),
            Some(80),
        ]);
        assert_eq!(predecessor_successor(&tree, 65), (Some(60), Some(70)));
    }
}
