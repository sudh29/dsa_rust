use crate::common::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let rev = reverse_list(head);
        assert_eq!(ListNode::to_vec(&rev), vec![5, 4, 3, 2, 1]);
    }
}
