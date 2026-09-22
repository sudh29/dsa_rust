use crate::common::ListNode;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut all_vals = Vec::new();
    for l in lists {
        all_vals.extend(ListNode::to_vec(&l));
    }
    all_vals.sort_unstable();
    ListNode::from_vec(&all_vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_k_lists() {
        let l1 = ListNode::from_vec(&[1, 4, 5]);
        let l2 = ListNode::from_vec(&[1, 3, 4]);
        let l3 = ListNode::from_vec(&[2, 6]);
        let merged = merge_k_lists(vec![l1, l2, l3]);
        assert_eq!(ListNode::to_vec(&merged), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }
}
