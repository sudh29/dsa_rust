use crate::common::ListNode;

pub fn add_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut v1 = ListNode::to_vec(&l1);
    let mut v2 = ListNode::to_vec(&l2);
    v1.reverse();
    v2.reverse();

    let mut res = Vec::new();
    let mut carry = 0;
    let n = v1.len().max(v2.len());

    for i in 0..n {
        let d1 = if i < v1.len() { v1[i] } else { 0 };
        let d2 = if i < v2.len() { v2[i] } else { 0 };
        let sum = d1 + d2 + carry;
        res.push(sum % 10);
        carry = sum / 10;
    }
    if carry > 0 {
        res.push(carry);
    }
    res.reverse();
    ListNode::from_vec(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_lists() {
        let l1 = ListNode::from_vec(&[4, 5]);
        let l2 = ListNode::from_vec(&[3, 4, 5]);
        assert_eq!(ListNode::to_vec(&add_two_lists(l1, l2)), vec![3, 9, 0]);
    }
}
