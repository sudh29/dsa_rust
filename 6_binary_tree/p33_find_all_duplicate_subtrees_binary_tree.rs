use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn find_duplicate_subtrees(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut res = Vec::new();

    fn serialize(
        node: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<String, usize>,
        res: &mut Vec<i32>,
    ) -> String {
        match node {
            Some(n) => {
                let nb = n.borrow();
                let s = format!(
                    "{}({})({})",
                    nb.val,
                    serialize(&nb.left, map, res),
                    serialize(&nb.right, map, res)
                );
                let count = map.entry(s.clone()).or_insert(0);
                *count += 1;
                if *count == 2 {
                    res.push(nb.val);
                }
                s
            }
            None => "#".to_string(),
        }
    }
    serialize(root, &mut map, &mut res);
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dup_subtrees() {
        let tree = TreeNode::from_level_order(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(2),
            Some(4),
            None,
            None,
            Some(4),
        ]);
        let dups = find_duplicate_subtrees(&tree);
        assert!(dups.contains(&4));
    }
}
