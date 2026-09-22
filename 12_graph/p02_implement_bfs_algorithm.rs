use std::collections::VecDeque;

pub fn bfs_of_graph(v: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    if v == 0 {
        return Vec::new();
    }
    let mut visited = vec![false; v];
    let mut res = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(0);
    visited[0] = true;

    while let Some(u) = queue.pop_front() {
        res.push(u);
        for &neighbor in &adj[u] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let adj = vec![vec![1, 2, 3], vec![], vec![4], vec![], vec![]];
        assert_eq!(bfs_of_graph(5, &adj), vec![0, 1, 2, 3, 4]);
    }
}
