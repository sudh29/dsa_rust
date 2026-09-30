use crate::common::ListNode;

/// Reverses a singly linked list in-place by updating pointer directions.
///
/// # Complexity
/// - Time Complexity: $O(N)$
/// - Space Complexity: $O(1)$ auxiliary space
///
/// # Examples
/// ```
/// use dsa_rust::linklist::p00_reverse_a_linked_list::reverse_list;
/// use dsa_rust::common::list_node::ListNode;
///
/// let head = ListNode::from_vec(&[1, 2, 3]);
/// let rev = reverse_list(head);
/// assert_eq!(ListNode::to_vec(&rev), vec![3, 2, 1]);
/// ```
pub fn reverse_list<T>(head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
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

        let empty: Option<Box<ListNode<i32>>> = None;
        assert_eq!(reverse_list(empty), None);

        let single = ListNode::from_vec(&[42]);
        assert_eq!(ListNode::to_vec(&reverse_list(single)), vec![42]);

        // Generic with strings
        let str_list = ListNode::from_slice(&["first", "second"]);
        let str_rev = reverse_list(str_list);
        assert_eq!(ListNode::to_vec_generic(&str_rev), vec!["second", "first"]);
    }
}
