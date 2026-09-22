use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn dup_sub(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut map: HashMap<String, usize> = HashMap::new();
    fn serialize(node: &Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<String, usize>) -> String {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let s = format!(
                    "{}({})({})",
                    nb.val,
                    serialize(&nb.left, map),
                    serialize(&nb.right, map)
                );
                if nb.left.is_some() || nb.right.is_some() {
                    *map.entry(s.clone()).or_insert(0) += 1;
                }
                s
            }
            None => "#".to_string(),
        }
    }
    serialize(root, &mut map);
    map.values().any(|&count| count > 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dup_sub() {
        let tree = TreeNode::from_level_order(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(2),
            None,
        ]);
        assert!(!dup_sub(&tree));
    }
}
