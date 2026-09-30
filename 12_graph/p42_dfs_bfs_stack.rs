use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs_iterative(graph: &HashMap<i32, Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut res = Vec::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        res.push(vertex);
        if let Some(neighbors) = graph.get(&vertex) {
            for &n in neighbors {
                if visited.insert(n) {
                    queue.push_back(n);
                }
            }
        }
    }
    res
}

pub fn dfs_iterative(graph: &HashMap<i32, Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut res = Vec::new();

    visited.insert(start);
    stack.push(start);

    while let Some(vertex) = stack.pop() {
        res.push(vertex);
        if let Some(neighbors) = graph.get(&vertex) {
            for &n in neighbors.iter().rev() {
                if visited.insert(n) {
                    stack.push(n);
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterative_traversals() {
        let mut g = HashMap::new();
        g.insert(1, vec![2, 3]);
        g.insert(2, vec![4]);
        g.insert(3, vec![5]);
        g.insert(4, vec![]);
        g.insert(5, vec![]);

        assert_eq!(bfs_iterative(&g, 1), vec![1, 2, 3, 4, 5]);
        assert_eq!(dfs_iterative(&g, 1), vec![1, 2, 4, 3, 5]);
    }
}
