/// A generic Segment Tree supporting point updates and range queries in $O(\log N)$.
///
/// # Complexity
/// - Build: $O(N)$
/// - Query: $O(\log N)$
/// - Point Update: $O(\log N)$
/// - Space: $O(N)$
///
/// # Examples
/// ```
/// use dsa_rust::common::segment_tree::SegmentTree;
///
/// // Range Sum Query Segment Tree
/// let mut st = SegmentTree::new(&[1, 3, 5, 7, 9, 11], 0, |a, b| a + b);
/// assert_eq!(st.query(1, 3), 15); // 3 + 5 + 7
///
/// st.update(1, 10);
/// assert_eq!(st.query(1, 3), 22); // 10 + 5 + 7
/// ```
#[derive(Debug, Clone)]
pub struct SegmentTree<T, F> {
    n: usize,
    tree: Vec<T>,
    default: T,
    op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> SegmentTree<T, F> {
    /// Constructs a `SegmentTree` from a slice of elements using an associative operator `op`.
    pub fn new(data: &[T], default: T, op: F) -> Self {
        let n = data.len();
        if n == 0 {
            return SegmentTree {
                n: 0,
                tree: Vec::new(),
                default,
                op,
            };
        }
        let mut tree = vec![default.clone(); 4 * n];
        Self::build(data, &mut tree, 1, 0, n - 1, &op);
        SegmentTree {
            n,
            tree,
            default,
            op,
        }
    }

    fn build(data: &[T], tree: &mut [T], node: usize, start: usize, end: usize, op: &F) {
        if start == end {
            tree[node] = data[start].clone();
            return;
        }
        let mid = start + (end - start) / 2;
        let left_child = 2 * node;
        let right_child = 2 * node + 1;
        Self::build(data, tree, left_child, start, mid, op);
        Self::build(data, tree, right_child, mid + 1, end, op);
        tree[node] = op(&tree[left_child], &tree[right_child]);
    }

    /// Queries the range `[left, right]` (0-indexed, inclusive).
    pub fn query(&self, left: usize, right: usize) -> T {
        if self.n == 0 || left > right || right >= self.n {
            return self.default.clone();
        }
        self.query_internal(1, 0, self.n - 1, left, right)
    }

    fn query_internal(
        &self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
    ) -> T {
        if left > end || right < start {
            return self.default.clone();
        }
        if left <= start && end <= right {
            return self.tree[node].clone();
        }
        let mid = start + (end - start) / 2;
        let left_val = self.query_internal(2 * node, start, mid, left, right);
        let right_val = self.query_internal(2 * node + 1, mid + 1, end, left, right);
        (self.op)(&left_val, &right_val)
    }

    /// Updates the element at `index` (0-indexed) with `value`.
    pub fn update(&mut self, index: usize, value: T) {
        if index >= self.n {
            return;
        }
        self.update_internal(1, 0, self.n - 1, index, value);
    }

    fn update_internal(&mut self, node: usize, start: usize, end: usize, index: usize, value: T) {
        if start == end {
            self.tree[node] = value;
            return;
        }
        let mid = start + (end - start) / 2;
        let left_child = 2 * node;
        let right_child = 2 * node + 1;
        if index <= mid {
            self.update_internal(left_child, start, mid, index, value);
        } else {
            self.update_internal(right_child, mid + 1, end, index, value);
        }
        self.tree[node] = (self.op)(&self.tree[left_child], &self.tree[right_child]);
    }

    /// Returns the number of elements in the underlying array.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns `true` if the segment tree represents an empty array.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_tree_sum() {
        let arr = [1, 3, 5, 7, 9, 11];
        let mut st = SegmentTree::new(&arr, 0, |a, b| a + b);
        assert_eq!(st.query(1, 3), 15);
        assert_eq!(st.query(0, 5), 36);

        st.update(1, 10);
        assert_eq!(st.query(1, 3), 22);
        assert_eq!(st.query(0, 1), 11);
    }

    #[test]
    fn test_segment_tree_min() {
        let arr = [5, 2, 8, 1, 9, 3];
        let mut st = SegmentTree::new(&arr, i32::MAX, |&a, &b| a.min(b));
        assert_eq!(st.query(0, 2), 2);
        assert_eq!(st.query(0, 5), 1);
        assert_eq!(st.query(4, 5), 3);

        st.update(3, 10);
        assert_eq!(st.query(0, 5), 2);
    }

    #[test]
    fn test_segment_tree_empty() {
        let st = SegmentTree::<i32, _>::new(&[], 0, |a, b| a + b);
        assert!(st.is_empty());
        assert_eq!(st.query(0, 0), 0);
    }
}
