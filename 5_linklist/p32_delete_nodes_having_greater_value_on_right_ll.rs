use crate::common::ListNode;

pub fn delete_smaller_nodes_on_right(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = ListNode::to_vec(&head);
    vals.reverse();
    let mut filtered = Vec::new();
    let mut max_val = i32::MIN;

    for x in vals {
        if x >= max_val {
            max_val = x;
            filtered.push(x);
        }
    }
    filtered.reverse();
    ListNode::from_vec(&filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_smaller() {
        let head = ListNode::from_vec(&[12, 15, 10, 11, 5, 6, 2, 3]);
        let res = delete_smaller_nodes_on_right(head);
        assert_eq!(ListNode::to_vec(&res), vec![15, 11, 6, 3]);
    }
}
