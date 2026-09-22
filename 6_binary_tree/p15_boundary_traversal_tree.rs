use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn boundary_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let root_node = match root {
        Some(r) => Rc::clone(r),
        None => return res,
    };
    let is_leaf = |n: &Rc<RefCell<TreeNode>>| -> bool {
        let b = n.borrow();
        b.left.is_none() && b.right.is_none()
    };

    if !is_leaf(&root_node) {
        res.push(root_node.borrow().val);
    }

    // left boundary
    let mut curr = root_node.borrow().left.as_ref().map(Rc::clone);
    while let Some(n) = curr {
        if !is_leaf(&n) {
            res.push(n.borrow().val);
        }
        let nb = n.borrow();
        curr = nb.left.as_ref().or(nb.right.as_ref()).map(Rc::clone);
    }

    // leaves
    fn add_leaves(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            let nb = n.borrow();
            if nb.left.is_none() && nb.right.is_none() {
                res.push(nb.val);
            } else {
                add_leaves(&nb.left, res);
                add_leaves(&nb.right, res);
            }
        }
    }
    add_leaves(root, &mut res);

    // right boundary in reverse
    let mut right_nodes = Vec::new();
    let mut curr_r = root_node.borrow().right.as_ref().map(Rc::clone);
    while let Some(n) = curr_r {
        if !is_leaf(&n) {
            right_nodes.push(n.borrow().val);
        }
        let nb = n.borrow();
        curr_r = nb.right.as_ref().or(nb.left.as_ref()).map(Rc::clone);
    }
    right_nodes.reverse();
    res.extend(right_nodes);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(boundary_traversal(&tree), vec![1, 2, 3]);
    }
}
