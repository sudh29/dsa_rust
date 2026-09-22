use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn mirror_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let mut nb = node.borrow_mut();
        mirror_tree(&nb.left);
        mirror_tree(&nb.right);
        let left = nb.left.take();
        nb.left = nb.right.take();
        nb.right = left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        mirror_tree(&tree);
        assert_eq!(TreeNode::to_inorder(&tree), vec![3, 1, 2]);
    }
}
