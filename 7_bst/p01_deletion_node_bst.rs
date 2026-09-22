use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut vals = TreeNode::to_inorder(&root);
    if let Some(pos) = vals.iter().position(|&x| x == key) {
        vals.remove(pos);
    }
    sorted_array_to_bst(&vals)
}

fn sorted_array_to_bst(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() {
        return None;
    }
    let mid = vals.len() / 2;
    let root = TreeNode::new_rc(vals[mid]);
    root.borrow_mut().left = sorted_array_to_bst(&vals[..mid]);
    root.borrow_mut().right = sorted_array_to_bst(&vals[mid + 1..]);
    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_bst() {
        let tree = TreeNode::from_level_order(&[
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        let new_tree = delete_node(tree, 3);
        assert_eq!(TreeNode::to_inorder(&new_tree), vec![2, 4, 5, 6, 7]);
    }
}
