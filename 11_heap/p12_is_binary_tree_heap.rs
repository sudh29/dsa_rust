use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_heap(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn count_nodes(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match node {
            Some(n) => 1 + count_nodes(&n.borrow().left) + count_nodes(&n.borrow().right),
            None => 0,
        }
    }

    fn is_complete(node: &Option<Rc<RefCell<TreeNode>>>, idx: usize, total: usize) -> bool {
        match node {
            Some(n) => {
                if idx >= total {
                    return false;
                }
                is_complete(&n.borrow().left, 2 * idx + 1, total)
                    && is_complete(&n.borrow().right, 2 * idx + 2, total)
            }
            None => true,
        }
    }

    fn is_heap_prop(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match node {
            Some(n) => {
                let nb = n.borrow();
                if let Some(ref l) = nb.left {
                    if nb.val < l.borrow().val {
                        return false;
                    }
                }
                if let Some(ref r) = nb.right {
                    if nb.val < r.borrow().val {
                        return false;
                    }
                }
                is_heap_prop(&nb.left) && is_heap_prop(&nb.right)
            }
            None => true,
        }
    }

    let total = count_nodes(root);
    is_complete(root, 0, total) && is_heap_prop(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_heap() {
        let tree = TreeNode::from_level_order(&[
            Some(97),
            Some(46),
            Some(37),
            Some(12),
            Some(3),
            Some(7),
            Some(31),
        ]);
        assert!(is_heap(&tree));
    }
}
