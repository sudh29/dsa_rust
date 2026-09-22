use crate::common::ListNode;

#[derive(Default)]
pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn append(&mut self, val: i32) {
        let new_node = Some(Box::new(ListNode::new(val)));
        if self.head.is_none() {
            self.head = new_node;
            return;
        }
        let mut curr = self.head.as_mut().unwrap();
        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = new_node;
    }

    pub fn delete_node(&mut self, val: i32) {
        let mut curr = &mut self.head;
        loop {
            match curr {
                Some(node) if node.val == val => {
                    *curr = node.next.take();
                    return;
                }
                Some(node) => {
                    curr = &mut node.next;
                }
                None => return,
            }
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        ListNode::to_vec(&self.head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ll() {
        let mut list = LinkedList::new();
        list.append(6);
        list.push(7);
        list.push(1);
        list.append(4);
        assert_eq!(list.to_vec(), vec![1, 7, 6, 4]);
        list.delete_node(7);
        assert_eq!(list.to_vec(), vec![1, 6, 4]);
    }
}
