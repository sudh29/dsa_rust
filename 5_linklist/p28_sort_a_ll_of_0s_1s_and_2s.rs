use crate::common::ListNode;

pub fn sort_012_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = ListNode::to_vec(&head);
    vals.sort_unstable();
    ListNode::from_vec(&vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_012_list() {
        let head = ListNode::from_vec(&[1, 2, 2, 1, 2, 0, 2, 2]);
        let sorted = sort_012_list(head);
        assert_eq!(ListNode::to_vec(&sorted), vec![0, 1, 1, 2, 2, 2, 2, 2]);
    }
}
