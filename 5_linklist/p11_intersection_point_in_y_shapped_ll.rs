use crate::common::ListNode;

pub fn intersect_point_values(
    l1: Option<&ListNode>,
    l2: Option<&ListNode>,
    common_suffix_len: usize,
) -> Option<i32> {
    let mut v1 = Vec::new();
    let mut curr = l1;
    while let Some(n) = curr {
        v1.push(n.val);
        curr = n.next.as_deref();
    }
    let mut v2 = Vec::new();
    let mut curr2 = l2;
    while let Some(n) = curr2 {
        v2.push(n.val);
        curr2 = n.next.as_deref();
    }
    if v1.len() < common_suffix_len || v2.len() < common_suffix_len || common_suffix_len == 0 {
        return None;
    }
    let idx1 = v1.len() - common_suffix_len;
    let idx2 = v2.len() - common_suffix_len;
    if v1[idx1..] == v2[idx2..] {
        Some(v1[idx1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_point() {
        let l1 = ListNode::from_vec(&[4, 1, 8, 4, 5]);
        let l2 = ListNode::from_vec(&[5, 6, 1, 8, 4, 5]);
        assert_eq!(
            intersect_point_values(l1.as_deref(), l2.as_deref(), 3),
            Some(8)
        );
    }
}
