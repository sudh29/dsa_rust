use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// A generic Binary Tree node using shared references (`Rc<RefCell<...>>`) in 100% safe Rust.
///
/// Defaults to `i32` value type for maximum ergonomics, while supporting arbitrary types `<T>`.
///
/// # Complexity
/// - Node creation: $O(1)$
/// - Level-order construction: $O(N)$
/// - In-order traversal: $O(N)$
///
/// # Examples
/// ```
/// use dsa_rust::common::tree_node::TreeNode;
///
/// // Generic with strings
/// let tree = TreeNode::from_level_order_generic(&[Some("root"), Some("left"), Some("right")]);
/// assert_eq!(TreeNode::to_inorder_generic(&tree), vec!["left", "root", "right"]);
///
/// // Default i32 usage
/// let int_tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
/// assert_eq!(TreeNode::to_inorder(&int_tree), vec![2, 1, 3]);
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode<T = i32> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new_rc(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(val)))
    }
}

impl<T: Clone> TreeNode<T> {
    /// Constructs a binary tree from level-order optional values.
    pub fn from_level_order_generic(vals: &[Option<T>]) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = TreeNode::new_rc(vals[0].as_ref().unwrap().clone());
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() && !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            if i < vals.len() {
                if let Some(ref val) = vals[i] {
                    let left = TreeNode::new_rc(val.clone());
                    curr.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(ref val) = vals[i] {
                    let right = TreeNode::new_rc(val.clone());
                    curr.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    /// Performs an in-order depth-first traversal, collecting values into a vector.
    pub fn to_inorder_generic(root: &Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
        let mut result = Vec::new();
        fn dfs<T: Clone>(node: &Option<Rc<RefCell<TreeNode<T>>>>, res: &mut Vec<T>) {
            if let Some(n) = node {
                let n_borrow = n.borrow();
                dfs(&n_borrow.left, res);
                res.push(n_borrow.val.clone());
                dfs(&n_borrow.right, res);
            }
        }
        dfs(root, &mut result);
        result
    }
}

impl TreeNode<i32> {
    /// Backwards-compatible convenience constructor for `i32` level-order slices.
    pub fn from_level_order(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        Self::from_level_order_generic(vals)
    }

    /// Backwards-compatible in-order traversal returning `Vec<i32>`.
    pub fn to_inorder(root: &Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
        Self::to_inorder_generic(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_tree() {
        let tree = TreeNode::from_level_order_generic(&[Some("b"), Some("a"), Some("c")]);
        assert_eq!(TreeNode::to_inorder_generic(&tree), vec!["a", "b", "c"]);

        let empty: Option<Rc<RefCell<TreeNode<String>>>> = TreeNode::from_level_order_generic(&[]);
        assert_eq!(empty, None);
        assert!(TreeNode::to_inorder_generic(&empty).is_empty());
    }
}
