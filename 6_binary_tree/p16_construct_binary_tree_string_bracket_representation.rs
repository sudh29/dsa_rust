use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn tree_to_string(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
    match root {
        Some(n) => {
            let nb = n.borrow();
            let mut s = nb.val.to_string();
            if nb.left.is_some() || nb.right.is_some() {
                s.push('(');
                s.push_str(&tree_to_string(&nb.left));
                s.push(')');
            }
            if nb.right.is_some() {
                s.push('(');
                s.push_str(&tree_to_string(&nb.right));
                s.push(')');
            }
            s
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracket_rep() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4)]);
        assert_eq!(tree_to_string(&tree), "1(2(4))(3)");
    }
}
