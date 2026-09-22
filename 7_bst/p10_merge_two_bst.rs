use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn merge_two_bst(
    root1: &Option<Rc<RefCell<TreeNode>>>,
    root2: &Option<Rc<RefCell<TreeNode>>>,
) -> Vec<i32> {
    let mut v1 = TreeNode::to_inorder(root1);
    let v2 = TreeNode::to_inorder(root2);
    v1.extend(v2);
    v1.sort_unstable();
    v1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_bst() {
        let t1 = TreeNode::from_level_order(&[Some(5), Some(3), Some(6)]);
        let t2 = TreeNode::from_level_order(&[Some(2), Some(1), Some(4)]);
        assert_eq!(merge_two_bst(&t1, &t2), vec![1, 2, 3, 4, 5, 6]);
    }
}
