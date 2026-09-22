use crate::common::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    if k <= 1 || head.is_none() {
        return head;
    }
    let mut vals = ListNode::to_vec(&head);
    for chunk in vals.chunks_mut(k) {
        chunk.reverse();
    }
    ListNode::from_vec(&vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_k_group() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let rev = reverse_k_group(head, 3);
        assert_eq!(ListNode::to_vec(&rev), vec![3, 2, 1, 6, 5, 4, 8, 7]);
    }
}
