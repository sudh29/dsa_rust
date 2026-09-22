use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

pub fn diagonal_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let root = match root {
        Some(r) => Rc::clone(r),
        None => return Vec::new(),
    };
    let mut q: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
    q.push_back((root, 0));

    while let Some((node, d)) = q.pop_front() {
        let mut curr = Some(node);
        while let Some(n) = curr {
            let nb = n.borrow();
            map.entry(d).or_default().push(nb.val);
            if let Some(ref l) = nb.left {
                q.push_back((Rc::clone(l), d + 1));
            }
            curr = nb.right.as_ref().map(Rc::clone);
        }
    }
    let mut res = Vec::new();
    for v in map.into_values() {
        res.extend(v);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal() {
        let tree = TreeNode::from_level_order(&[
            Some(8),
            Some(3),
            Some(10),
            Some(1),
            Some(6),
            None,
            Some(14),
        ]);
        let diag = diagonal_traversal(&tree);
        assert_eq!(diag[0], 8);
        assert_eq!(diag[1], 10);
    }
}
