use crate::common::ListNode;

pub fn multiply_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> i64 {
    let modulo = 1_000_000_007i64;
    let mut n1 = 0i64;
    let mut n2 = 0i64;

    let mut curr = l1;
    while let Some(n) = curr {
        n1 = (n1 * 10 + n.val as i64) % modulo;
        curr = n.next;
    }
    let mut curr2 = l2;
    while let Some(n) = curr2 {
        n2 = (n2 * 10 + n.val as i64) % modulo;
        curr2 = n.next;
    }
    (n1 * n2) % modulo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_lists() {
        let l1 = ListNode::from_vec(&[3, 2]);
        let l2 = ListNode::from_vec(&[2]);
        assert_eq!(multiply_two_lists(l1, l2), 64);
    }
}
