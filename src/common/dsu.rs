/// Disjoint Set Union (DSU / Union-Find) with path compression and union-by-rank.
///
/// # Complexity
/// - `find`: $O(\alpha(N))$ amortized, where $\alpha$ is the inverse Ackermann function
/// - `union`: $O(\alpha(N))$ amortized
/// - `connected`: $O(\alpha(N))$ amortized
/// - Space: $O(N)$
///
/// # Examples
/// ```
/// use dsa_rust::common::dsu::DisjointSetUnion;
///
/// let mut dsu = DisjointSetUnion::new(5);
/// assert!(!dsu.connected(0, 1));
/// assert!(dsu.union(0, 1));
/// assert!(dsu.connected(0, 1));
/// assert_eq!(dsu.count(), 4);
/// ```
#[derive(Debug, Clone)]
pub struct DisjointSetUnion {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl DisjointSetUnion {
    /// Creates a new `DisjointSetUnion` with `size` elements (from `0` to `size - 1`).
    pub fn new(size: usize) -> Self {
        DisjointSetUnion {
            parent: (0..size).collect(),
            rank: vec![0; size],
            components: size,
        }
    }

    /// Finds the representative (root) of the set containing `x` with path compression.
    pub fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        // Path compression
        while x != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }
        root
    }

    /// Unites the sets containing elements `x` and `y` using union-by-rank.
    /// Returns `true` if `x` and `y` were in different sets, or `false` if already connected.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
        self.components -= 1;
        true
    }

    /// Checks if elements `x` and `y` belong to the same connected component.
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the number of disjoint connected components.
    pub fn count(&self) -> usize {
        self.components
    }

    /// Returns the total number of elements.
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Returns `true` if the DSU contains no elements.
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu_basic_operations() {
        let mut dsu = DisjointSetUnion::new(10);
        assert_eq!(dsu.count(), 10);
        assert!(!dsu.connected(1, 2));

        assert!(dsu.union(1, 2));
        assert!(dsu.connected(1, 2));
        assert_eq!(dsu.count(), 9);

        // Union already connected
        assert!(!dsu.union(1, 2));
        assert_eq!(dsu.count(), 9);

        assert!(dsu.union(2, 3));
        assert!(dsu.connected(1, 3));
        assert_eq!(dsu.count(), 8);

        // Transitive connectivity
        assert!(dsu.union(4, 5));
        assert!(dsu.union(5, 6));
        assert!(dsu.union(1, 6));
        assert!(dsu.connected(3, 4));
    }

    #[test]
    fn test_dsu_empty() {
        let dsu = DisjointSetUnion::new(0);
        assert!(dsu.is_empty());
        assert_eq!(dsu.len(), 0);
    }
}
