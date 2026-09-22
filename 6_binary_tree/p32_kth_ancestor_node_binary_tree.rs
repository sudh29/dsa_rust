use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_ancestor(root: &Option<Rc<RefCell<TreeNode>>>, k: usize, target: i32) -> Option<i32> {
    let mut path = Vec::new();
    fn find_path(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, path: &mut Vec<i32>) -> bool {
        if let Some(n) = node {
            let nb = n.borrow();
            path.push(nb.val);
            if nb.val == target {
                return true;
            }
            if find_path(&nb.left, target, path) || find_path(&nb.right, target, path) {
                return true;
            }
            path.pop();
        }
        false
    }
    if find_path(root, target, &mut path) && path.len() > k {
        Some(path[path.len() - 1 - k])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_ancestor() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(kth_ancestor(&tree, 2, 4), Some(1));
    }
}
