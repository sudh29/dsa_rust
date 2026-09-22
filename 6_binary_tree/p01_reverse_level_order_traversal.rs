use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn reverse_level_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let root = match root {
        Some(r) => Rc::clone(r),
        None => return res,
    };
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        let node_borrow = node.borrow();
        res.push(node_borrow.val);
        if let Some(ref right) = node_borrow.right {
            queue.push_back(Rc::clone(right));
        }
        if let Some(ref left) = node_borrow.left {
            queue.push_back(Rc::clone(left));
        }
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_level_order() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(reverse_level_order(&tree), vec![2, 3, 1]);
    }
}
