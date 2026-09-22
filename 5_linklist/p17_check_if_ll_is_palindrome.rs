use crate::common::ListNode;

pub fn is_palindrome_list(head: &Option<Box<ListNode>>) -> bool {
    let vals = ListNode::to_vec(head);
    let n = vals.len();
    for i in 0..n / 2 {
        if vals[i] != vals[n - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_list() {
        let head = ListNode::from_vec(&[1, 2, 2, 1]);
        assert!(is_palindrome_list(&head));
        let head2 = ListNode::from_vec(&[1, 2, 3]);
        assert!(!is_palindrome_list(&head2));
    }
}
