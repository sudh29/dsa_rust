use crate::common::ListNode;

pub fn remove_duplicates_sorted(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr = head.as_mut();
    while let Some(node) = curr {
        while let Some(next) = node.next.as_mut() {
            if node.val == next.val {
                node.next = next.next.take();
            } else {
                break;
            }
        }
        curr = node.next.as_mut();
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dup_sorted() {
        let head = ListNode::from_vec(&[2, 2, 4, 5]);
        let res = remove_duplicates_sorted(head);
        assert_eq!(ListNode::to_vec(&res), vec![2, 4, 5]);
    }
}
