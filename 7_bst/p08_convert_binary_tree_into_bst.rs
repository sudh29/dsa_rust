use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn binary_tree_to_bst(root: &Option<Rc<RefCell<TreeNode>>>) {
    let mut vals = TreeNode::to_inorder(root);
    vals.sort_unstable();
    let mut idx = 0;
    fn fill_inorder(node: &Option<Rc<RefCell<TreeNode>>>, vals: &[i32], idx: &mut usize) {
        if let Some(n) = node {
            fill_inorder(&n.borrow().left, vals, idx);
            n.borrow_mut().val = vals[*idx];
            *idx += 1;
            fill_inorder(&n.borrow().right, vals, idx);
        }
    }
    fill_inorder(root, &vals, &mut idx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_to_bst() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        binary_tree_to_bst(&tree);
        assert_eq!(TreeNode::to_inorder(&tree), vec![1, 2, 3]);
    }
}
