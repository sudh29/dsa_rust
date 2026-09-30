/// A generic Singly-Linked List node in 100% safe Rust.
///
/// Defaults to `i32` value type for maximum ergonomics, while supporting arbitrary types `<T>`.
///
/// # Complexity
/// - Node creation: $O(1)$
/// - From slice: $O(N)$
/// - To vector: $O(N)$
///
/// # Examples
/// ```
/// use dsa_rust::common::list_node::ListNode;
///
/// // Generic with string slices
/// let list = ListNode::from_slice(&["a", "b", "c"]);
/// assert_eq!(ListNode::to_vec_generic(&list), vec!["a", "b", "c"]);
///
/// // Default i32 usage
/// let int_list = ListNode::from_vec(&[1, 2, 3]);
/// assert_eq!(ListNode::to_vec(&int_list), vec![1, 2, 3]);
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T = i32> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

impl<T: Clone> ListNode<T> {
    /// Constructs a linked list from any slice of cloneable items.
    pub fn from_slice(slice: &[T]) -> Option<Box<ListNode<T>>> {
        let mut head = None;
        for val in slice.iter().rev() {
            let mut node = Box::new(ListNode::new(val.clone()));
            node.next = head;
            head = Some(node);
        }
        head
    }

    /// Converts a linked list into a vector of cloneable items.
    pub fn to_vec_generic(head: &Option<Box<ListNode<T>>>) -> Vec<T> {
        let mut result = Vec::new();
        let mut curr = head;
        while let Some(node) = curr {
            result.push(node.val.clone());
            curr = &node.next;
        }
        result
    }
}

impl ListNode<i32> {
    /// Backwards-compatible convenience constructor for `i32` slices.
    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode<i32>>> {
        Self::from_slice(vec)
    }

    /// Backwards-compatible convenience conversion for `i32` lists.
    pub fn to_vec(head: &Option<Box<ListNode<i32>>>) -> Vec<i32> {
        Self::to_vec_generic(head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_list() {
        let list = ListNode::from_slice(&["x", "y", "z"]);
        assert_eq!(ListNode::to_vec_generic(&list), vec!["x", "y", "z"]);

        let empty: Option<Box<ListNode<i32>>> = ListNode::from_slice(&[]);
        assert_eq!(empty, None);
        assert!(ListNode::to_vec(&empty).is_empty());
    }
}
