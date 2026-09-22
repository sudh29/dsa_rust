use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn inorder_successors_list(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<(i32, Option<i32>)> {
    let vals = TreeNode::to_inorder(root);
    let mut res = Vec::new();
    for i in 0..vals.len() {
        let succ = if i + 1 < vals.len() {
            Some(vals[i + 1])
        } else {
            None
        };
        res.push((vals[i], succ));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate_succ() {
        let tree = TreeNode::from_level_order(&[Some(10), Some(8), Some(12), Some(3)]);
        let succs = inorder_successors_list(&tree);
        assert_eq!(
            succs,
            vec![(3, Some(8)), (8, Some(10)), (10, Some(12)), (12, None)]
        );
    }
}
