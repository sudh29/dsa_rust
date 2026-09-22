use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }
    let root_val = preorder[0];
    let root = TreeNode::new_rc(root_val);
    let root_idx = inorder.iter().position(|&x| x == root_val).unwrap();

    let left_in = &inorder[..root_idx];
    let right_in = &inorder[root_idx + 1..];

    let left_pre = &preorder[1..1 + left_in.len()];
    let right_pre = &preorder[1 + left_in.len()..];

    root.borrow_mut().left = build_tree(left_pre, left_in);
    root.borrow_mut().right = build_tree(right_pre, right_in);

    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        let pre = [3, 9, 20, 15, 7];
        let in_order = [9, 3, 15, 20, 7];
        let root = build_tree(&pre, &in_order);
        assert_eq!(TreeNode::to_inorder(&root), vec![9, 3, 15, 20, 7]);
    }
}
