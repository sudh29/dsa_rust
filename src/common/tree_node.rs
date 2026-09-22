use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new_rc(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(val)))
    }

    pub fn from_level_order(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = TreeNode::new_rc(vals[0].unwrap());
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() && !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let left = TreeNode::new_rc(val);
                    curr.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = TreeNode::new_rc(val);
                    curr.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    pub fn to_inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                let n_borrow = n.borrow();
                dfs(&n_borrow.left, res);
                res.push(n_borrow.val);
                dfs(&n_borrow.right, res);
            }
        }
        dfs(root, &mut result);
        result
    }
}
