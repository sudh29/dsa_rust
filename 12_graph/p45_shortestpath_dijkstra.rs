use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
struct NodeState {
    cost: i32,
    name: String,
}

impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path_dijkstra(
    graph: &HashMap<String, Vec<(String, i32)>>,
    src: &str,
    dest: &str,
) -> Option<(i32, Vec<String>)> {
    let mut dist = HashMap::new();
    let mut prev: HashMap<String, String> = HashMap::new();
    let mut pq = BinaryHeap::new();

    dist.insert(src.to_string(), 0);
    pq.push(NodeState {
        cost: 0,
        name: src.to_string(),
    });

    while let Some(NodeState { cost, name }) = pq.pop() {
        if name == dest {
            let mut path = Vec::new();
            let mut curr = dest.to_string();
            while let Some(p) = prev.get(&curr) {
                path.push(curr.clone());
                curr = p.clone();
            }
            path.push(src.to_string());
            path.reverse();
            return Some((cost, path));
        }

        if cost > *dist.get(&name).unwrap_or(&i32::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&name) {
            for (next_name, weight) in neighbors {
                let next_cost = cost + weight;
                if next_cost < *dist.get(next_name).unwrap_or(&i32::MAX) {
                    dist.insert(next_name.clone(), next_cost);
                    prev.insert(next_name.clone(), name.clone());
                    pq.push(NodeState {
                        cost: next_cost,
                        name: next_name.clone(),
                    });
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_path() {
        let mut g = HashMap::new();
        g.insert("a".into(), vec![("b".into(), 4), ("c".into(), 2)]);
        g.insert("b".into(), vec![("d".into(), 5)]);
        g.insert("c".into(), vec![("b".into(), 1), ("d".into(), 8)]);
        g.insert("d".into(), vec![]);

        let (cost, path) = shortest_path_dijkstra(&g, "a", "d").unwrap();
        assert_eq!(cost, 8);
        assert_eq!(path, vec!["a", "c", "b", "d"]);
    }
}
