use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn insert_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(n) => {
            if val < n.borrow().val {
                let left = n.borrow().left.as_ref().map(Rc::clone);
                n.borrow_mut().left = insert_bst(left, val);
            } else if val > n.borrow().val {
                let right = n.borrow().right.as_ref().map(Rc::clone);
                n.borrow_mut().right = insert_bst(right, val);
            }
            Some(n)
        }
        None => Some(TreeNode::new_rc(val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut root = insert_bst(None, 5);
        root = insert_bst(root, 3);
        root = insert_bst(root, 7);
        assert_eq!(TreeNode::to_inorder(&root), vec![3, 5, 7]);
    }
}
