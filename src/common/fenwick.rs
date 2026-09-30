/// Binary Indexed Tree (Fenwick Tree) for prefix sums and point updates in $O(\log N)$.
///
/// # Complexity
/// - `add`: $O(\log N)$
/// - `prefix_sum`: $O(\log N)$
/// - `range_sum`: $O(\log N)$
/// - Space: $O(N)$
///
/// # Examples
/// ```
/// use dsa_rust::common::fenwick::FenwickTree;
///
/// let mut ft = FenwickTree::from_slice(&[2, 1, 1, 3, 2, 3, 4, 5, 6, 7, 8, 9]);
/// assert_eq!(ft.prefix_sum(5), 12); // indices 0..=5
/// assert_eq!(ft.range_sum(1, 5), 10); // indices 1..=5
///
/// ft.add(3, 6); // Add 6 to index 3
/// assert_eq!(ft.range_sum(1, 5), 16);
/// ```
#[derive(Debug, Clone)]
pub struct FenwickTree {
    n: usize,
    tree: Vec<i64>,
}

impl FenwickTree {
    /// Creates an empty `FenwickTree` of capacity `n` initialized to zero.
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            tree: vec![0; n + 1],
        }
    }

    /// Constructs a `FenwickTree` from a slice of values.
    pub fn from_slice(values: &[i64]) -> Self {
        let n = values.len();
        let mut ft = FenwickTree::new(n);
        for (i, &val) in values.iter().enumerate() {
            ft.add(i, val);
        }
        ft
    }

    /// Adds `delta` to the element at `index` (0-indexed).
    pub fn add(&mut self, index: usize, delta: i64) {
        let mut i = index + 1;
        while i <= self.n {
            self.tree[i] += delta;
            i += i & (!i + 1); // i & -i in two's complement
        }
    }

    /// Computes the prefix sum for the range `[0, index]` (0-indexed, inclusive).
    pub fn prefix_sum(&self, index: usize) -> i64 {
        if index >= self.n {
            return self.prefix_sum(self.n.saturating_sub(1));
        }
        let mut sum = 0;
        let mut i = index + 1;
        while i > 0 {
            sum += self.tree[i];
            i -= i & (!i + 1);
        }
        sum
    }

    /// Computes the sum of elements in the range `[left, right]` (0-indexed, inclusive).
    pub fn range_sum(&self, left: usize, right: usize) -> i64 {
        if left > right || self.n == 0 {
            return 0;
        }
        let right_sum = self.prefix_sum(right);
        if left == 0 {
            right_sum
        } else {
            right_sum - self.prefix_sum(left - 1)
        }
    }

    /// Returns the number of elements tracked by the Fenwick tree.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns `true` if the Fenwick tree is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let arr = [1, 2, 3, 4, 5];
        let mut ft = FenwickTree::from_slice(&arr);

        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(2), 6);
        assert_eq!(ft.prefix_sum(4), 15);
        assert_eq!(ft.range_sum(1, 3), 9); // 2 + 3 + 4

        ft.add(2, 5); // 3 becomes 8
        assert_eq!(ft.range_sum(1, 3), 14); // 2 + 8 + 4
        assert_eq!(ft.prefix_sum(4), 20);
    }

    #[test]
    fn test_fenwick_empty() {
        let ft = FenwickTree::new(0);
        assert!(ft.is_empty());
        assert_eq!(ft.range_sum(0, 0), 0);
    }
}
