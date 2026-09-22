#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    pub fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut curr = head;
        while let Some(node) = curr {
            result.push(node.val);
            curr = &node.next;
        }
        result
    }
}
