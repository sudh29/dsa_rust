use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

pub fn bottom_view(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    let root = match root {
        Some(r) => Rc::clone(r),
        None => return Vec::new(),
    };
    let mut q: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
    q.push_back((root, 0));

    while let Some((node, hd)) = q.pop_front() {
        let nb = node.borrow();
        map.insert(hd, nb.val);
        if let Some(ref l) = nb.left {
            q.push_back((Rc::clone(l), hd - 1));
        }
        if let Some(ref r) = nb.right {
            q.push_back((Rc::clone(r), hd + 1));
        }
    }
    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_view() {
        let tree = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(bottom_view(&tree), vec![2, 1, 3]);
    }
}
