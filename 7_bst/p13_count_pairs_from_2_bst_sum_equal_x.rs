use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn count_pairs(
    root1: &Option<Rc<RefCell<TreeNode>>>,
    root2: &Option<Rc<RefCell<TreeNode>>>,
    x: i32,
) -> usize {
    let v1 = TreeNode::to_inorder(root1);
    let v2: HashSet<i32> = TreeNode::to_inorder(root2).into_iter().collect();
    let mut count = 0;
    for a in v1 {
        if v2.contains(&(x - a)) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_pairs() {
        let t1 = TreeNode::from_level_order(&[Some(5), Some(3), Some(7)]);
        let t2 = TreeNode::from_level_order(&[Some(10), Some(6), Some(15)]);
        assert_eq!(count_pairs(&t1, &t2, 15), 1);
    }
}
