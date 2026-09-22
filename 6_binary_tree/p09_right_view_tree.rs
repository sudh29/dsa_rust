use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn right_view(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let root = match root {
        Some(r) => Rc::clone(r),
        None => return res,
    };
    let mut q = VecDeque::new();
    q.push_back(root);

    while !q.is_empty() {
        let len = q.len();
        for i in 0..len {
            let node = q.pop_front().unwrap();
            let nb = node.borrow();
            if i == len - 1 {
                res.push(nb.val);
            }
            if let Some(ref l) = nb.left {
                q.push_back(Rc::clone(l));
            }
            if let Some(ref r) = nb.right {
                q.push_back(Rc::clone(r));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_view() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(right_view(&tree), vec![1, 3, 5]);
    }
}
