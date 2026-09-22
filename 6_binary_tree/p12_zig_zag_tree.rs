use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn zig_zag_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let root = match root {
        Some(r) => Rc::clone(r),
        None => return res,
    };
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut left_to_right = true;

    while !q.is_empty() {
        let len = q.len();
        let mut level = Vec::new();
        for _ in 0..len {
            let node = q.pop_front().unwrap();
            let nb = node.borrow();
            level.push(nb.val);
            if let Some(ref l) = nb.left {
                q.push_back(Rc::clone(l));
            }
            if let Some(ref r) = nb.right {
                q.push_back(Rc::clone(r));
            }
        }
        if !left_to_right {
            level.reverse();
        }
        res.extend(level);
        left_to_right = !left_to_right;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zig_zag() {
        let tree = TreeNode::from_level_order(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(zig_zag_traversal(&tree), vec![3, 20, 9, 15, 7]);
    }
}
