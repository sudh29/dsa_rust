use crate::common::ListNode;

pub fn middle_node(head: &Option<Box<ListNode>>) -> Option<i32> {
    let vals = ListNode::to_vec(head);
    if vals.is_empty() {
        None
    } else {
        Some(vals[vals.len() / 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(middle_node(&head), Some(3));
        let head2 = ListNode::from_vec(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(middle_node(&head2), Some(4));
    }
}
