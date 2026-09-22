use crate::common::ListNode;

pub fn segregate_even_odd(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let vals = ListNode::to_vec(&head);
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    for x in vals {
        if x % 2 == 0 {
            evens.push(x);
        } else {
            odds.push(x);
        }
    }
    evens.extend(odds);
    ListNode::from_vec(&evens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segregate_even_odd() {
        let head = ListNode::from_vec(&[17, 15, 8, 9, 2, 4, 6]);
        let res = segregate_even_odd(head);
        assert_eq!(ListNode::to_vec(&res), vec![8, 2, 4, 6, 17, 15, 9]);
    }
}
