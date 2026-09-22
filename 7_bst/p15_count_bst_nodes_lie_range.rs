use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_count_in_range(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, h: i32) -> usize {
    let vals = TreeNode::to_inorder(root);
    vals.into_iter().filter(|&x| x >= l && x <= h).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_range() {
        let tree = TreeNode::from_level_order(&[
            Some(10),
            Some(5),
            Some(50),
            Some(1),
            None,
            Some(40),
            Some(100),
        ]);
        assert_eq!(get_count_in_range(&tree, 5, 45), 3);
    }
}
