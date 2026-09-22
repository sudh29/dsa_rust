use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn k_sum_paths_count(root: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> usize {
    let mut count = 0;
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, k: i32, count: &mut usize) {
        if let Some(n) = node {
            let nb = n.borrow();
            path.push(nb.val);
            let mut sum = 0;
            for &val in path.iter().rev() {
                sum += val;
                if sum == k {
                    *count += 1;
                }
            }
            dfs(&nb.left, path, k, count);
            dfs(&nb.right, path, k, count);
            path.pop();
        }
    }
    let mut path = Vec::new();
    dfs(root, &mut path, k, &mut count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_sum_paths() {
        let tree = TreeNode::from_level_order(&[
            Some(1),
            Some(3),
            Some(-1),
            Some(2),
            Some(1),
            Some(4),
            Some(5),
        ]);
        assert_eq!(k_sum_paths_count(&tree, 5), 4);
    }
}
