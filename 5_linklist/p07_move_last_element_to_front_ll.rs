use crate::common::ListNode;

pub fn move_to_front(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = ListNode::to_vec(&head);
    if vals.len() <= 1 {
        return head;
    }
    let last = vals.pop().unwrap();
    vals.insert(0, last);
    ListNode::from_vec(&vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to_front() {
        let head = ListNode::from_vec(&[2, 5, 6, 2, 1]);
        let res = move_to_front(head);
        assert_eq!(ListNode::to_vec(&res), vec![1, 2, 5, 6, 2]);
    }
}
