use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn bst_to_min_heap(root: &Option<Rc<RefCell<TreeNode>>>) {
    let vals = TreeNode::to_inorder(root);
    let mut idx = 0;
    fn fill_preorder(node: &Option<Rc<RefCell<TreeNode>>>, vals: &[i32], idx: &mut usize) {
        if let Some(n) = node {
            n.borrow_mut().val = vals[*idx];
            *idx += 1;
            fill_preorder(&n.borrow().left, vals, idx);
            fill_preorder(&n.borrow().right, vals, idx);
        }
    }
    fill_preorder(root, &vals, &mut idx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_to_min_heap() {
        let tree = TreeNode::from_level_order(&[
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
        ]);
        bst_to_min_heap(&tree);
        assert_eq!(tree.as_ref().unwrap().borrow().val, 1);
    }
}
