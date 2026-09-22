use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn level_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let root = match root {
        Some(r) => Rc::clone(r),
        None => return res,
    };
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::new();
        for _ in 0..level_size {
            let curr = queue.pop_front().unwrap();
            let curr_borrow = curr.borrow();
            level.push(curr_borrow.val);
            if let Some(ref left) = curr_borrow.left {
                queue.push_back(Rc::clone(left));
            }
            if let Some(ref right) = curr_borrow.right {
                queue.push_back(Rc::clone(right));
            }
        }
        res.push(level);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order() {
        let tree = TreeNode::from_level_order(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(level_order(&tree), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}
