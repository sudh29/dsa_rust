use crate::common::ListNode;

pub fn add_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vals = ListNode::to_vec(&head);
    let mut carry = 1;

    for val in vals.iter_mut().rev() {
        let sum = *val + carry;
        *val = sum % 10;
        carry = sum / 10;
        if carry == 0 {
            break;
        }
    }
    if carry > 0 {
        vals.insert(0, carry);
    }
    ListNode::from_vec(&vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let head = ListNode::from_vec(&[4, 5, 6]);
        assert_eq!(ListNode::to_vec(&add_one(head)), vec![4, 5, 7]);
        let head2 = ListNode::from_vec(&[9, 9, 9]);
        assert_eq!(ListNode::to_vec(&add_one(head2)), vec![1, 0, 0, 0]);
    }
}
