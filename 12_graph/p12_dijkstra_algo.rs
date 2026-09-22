use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(v: usize, adj: &[Vec<(usize, i32)>], s: usize) -> Vec<i32> {
    let mut dist = vec![i32::MAX; v];
    let mut heap = BinaryHeap::new();

    dist[s] = 0;
    heap.push(State {
        cost: 0,
        position: s,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }
        for &(next_pos, weight) in &adj[position] {
            let next_cost = cost + weight;
            if next_cost < dist[next_pos] {
                dist[next_pos] = next_cost;
                heap.push(State {
                    cost: next_cost,
                    position: next_pos,
                });
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let adj = vec![vec![(1, 9)], vec![(0, 9)]];
        assert_eq!(dijkstra(2, &adj, 0), vec![0, 9]);

        let adj2 = vec![
            vec![(1, 1), (2, 6)],
            vec![(2, 2), (0, 1)],
            vec![(1, 2), (0, 6)],
        ];
        assert_eq!(dijkstra(3, &adj2, 2), vec![3, 2, 0]);
    }
}
