use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn max_level_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root = match root {
        Some(r) => Rc::clone(r),
        None => return 0,
    };
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut max_sum = i32::MIN;

    while !q.is_empty() {
        let len = q.len();
        let mut level_sum = 0;
        for _ in 0..len {
            let node = q.pop_front().unwrap();
            let nb = node.borrow();
            level_sum += nb.val;
            if let Some(ref l) = nb.left {
                q.push_back(Rc::clone(l));
            }
            if let Some(ref r) = nb.right {
                q.push_back(Rc::clone(r));
            }
        }
        max_sum = max_sum.max(level_sum);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_level_sum() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(7), Some(0), Some(7), Some(-8)]);
        assert_eq!(max_level_sum(&tree), 7);
    }
}
