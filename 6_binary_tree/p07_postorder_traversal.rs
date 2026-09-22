use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn postorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            let nb = n.borrow();
            dfs(&nb.left, res);
            dfs(&nb.right, res);
            res.push(nb.val);
        }
    }
    dfs(root, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postorder() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(postorder(&tree), vec![2, 3, 1]);
    }
}
