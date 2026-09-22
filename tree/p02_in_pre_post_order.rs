use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn traversals(root: &Option<Rc<RefCell<TreeNode>>>) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let inorder = TreeNode::to_inorder(root);
    let preorder = crate::binary_tree::p06_preorder_traversal::preorder(root);
    let postorder = crate::binary_tree::p07_postorder_traversal::postorder(root);
    (inorder, preorder, postorder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversals() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        let (i, pr, po) = traversals(&tree);
        assert_eq!(i, vec![2, 1, 3]);
        assert_eq!(pr, vec![1, 2, 3]);
        assert_eq!(po, vec![2, 3, 1]);
    }
}
