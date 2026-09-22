use crate::common::ListNode;

pub fn get_nth_from_last(head: &Option<Box<ListNode>>, n: usize) -> i32 {
    let vals = ListNode::to_vec(head);
    if n == 0 || n > vals.len() {
        -1
    } else {
        vals[vals.len() - n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_from_last() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(get_nth_from_last(&head, 2), 8);
        assert_eq!(get_nth_from_last(&head, 10), -1);
    }
}
