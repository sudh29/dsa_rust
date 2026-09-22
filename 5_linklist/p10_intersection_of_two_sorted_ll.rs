use crate::common::ListNode;

pub fn find_intersection(
    head1: Option<Box<ListNode>>,
    head2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let v1 = ListNode::to_vec(&head1);
    let v2 = ListNode::to_vec(&head2);
    let (mut i, mut j) = (0, 0);
    let mut res = Vec::new();

    while i < v1.len() && j < v2.len() {
        if v1[i] == v2[j] {
            res.push(v1[i]);
            i += 1;
            j += 1;
        } else if v1[i] < v2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    ListNode::from_vec(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let l1 = ListNode::from_vec(&[1, 2, 3, 4, 6]);
        let l2 = ListNode::from_vec(&[2, 4, 6, 8]);
        assert_eq!(ListNode::to_vec(&find_intersection(l1, l2)), vec![2, 4, 6]);
    }
}
