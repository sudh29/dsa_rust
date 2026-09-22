use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    weight: i32,
    u: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn spanning_tree_prim(v: usize, adj: &[Vec<(usize, i32)>]) -> i32 {
    let mut in_mst = vec![false; v];
    let mut pq = BinaryHeap::new();
    let mut mst_weight = 0;

    pq.push(Edge { weight: 0, u: 0 });

    while let Some(Edge { weight, u }) = pq.pop() {
        if in_mst[u] {
            continue;
        }
        in_mst[u] = true;
        mst_weight += weight;

        for &(v_next, w) in &adj[u] {
            if !in_mst[v_next] {
                pq.push(Edge {
                    weight: w,
                    u: v_next,
                });
            }
        }
    }
    mst_weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mst() {
        let adj = vec![
            vec![(1, 5), (2, 1)],
            vec![(0, 5), (2, 3)],
            vec![(0, 1), (1, 3)],
        ];
        assert_eq!(spanning_tree_prim(3, &adj), 4);
    }
}
