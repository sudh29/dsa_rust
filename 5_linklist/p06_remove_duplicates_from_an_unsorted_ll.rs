use crate::common::ListNode;
use std::collections::HashSet;

pub fn remove_duplicates_unsorted(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut seen = HashSet::new();
    let mut vals = Vec::new();
    let mut curr = head;

    while let Some(node) = curr {
        if seen.insert(node.val) {
            vals.push(node.val);
        }
        curr = node.next;
    }
    ListNode::from_vec(&vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dup_unsorted() {
        let head = ListNode::from_vec(&[5, 2, 2, 4]);
        let res = remove_duplicates_unsorted(head);
        assert_eq!(ListNode::to_vec(&res), vec![5, 2, 4]);
    }
}
