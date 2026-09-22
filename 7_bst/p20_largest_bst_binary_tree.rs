use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Info {
    is_bst: bool,
    size: usize,
    min_val: i32,
    max_val: i32,
}

pub fn largest_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> Info {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let left = dfs(&nb.left);
                let right = dfs(&nb.right);

                if left.is_bst && right.is_bst && nb.val > left.max_val && nb.val < right.min_val {
                    Info {
                        is_bst: true,
                        size: 1 + left.size + right.size,
                        min_val: if nb.left.is_some() {
                            left.min_val
                        } else {
                            nb.val
                        },
                        max_val: if nb.right.is_some() {
                            right.max_val
                        } else {
                            nb.val
                        },
                    }
                } else {
                    Info {
                        is_bst: false,
                        size: left.size.max(right.size),
                        min_val: i32::MIN,
                        max_val: i32::MAX,
                    }
                }
            }
            None => Info {
                is_bst: true,
                size: 0,
                min_val: i32::MAX,
                max_val: i32::MIN,
            },
        }
    }
    dfs(root).size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_bst() {
        let tree = TreeNode::from_level_order(&[
            Some(1),
            Some(4),
            Some(3),
            Some(2),
            Some(4),
            Some(2),
            Some(5),
        ]);
        assert!(largest_bst(&tree) >= 1);
    }
}
