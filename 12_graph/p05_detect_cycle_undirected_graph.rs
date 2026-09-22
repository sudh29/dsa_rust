pub fn is_cycle_undirected(v: usize, adj: &[Vec<usize>]) -> bool {
    let mut visited = vec![false; v];

    fn dfs(u: usize, parent: usize, adj: &[Vec<usize>], visited: &mut [bool]) -> bool {
        visited[u] = true;
        for &neighbor in &adj[u] {
            if !visited[neighbor] {
                if dfs(neighbor, u, adj, visited) {
                    return true;
                }
            } else if neighbor != parent {
                return true;
            }
        }
        false
    }

    for i in 0..v {
        if !visited[i] && dfs(i, usize::MAX, adj, &mut visited) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_undirected() {
        let adj = vec![vec![1], vec![0, 2, 4], vec![1, 3], vec![2, 4], vec![1, 3]];
        assert!(is_cycle_undirected(5, &adj));

        let adj2 = vec![vec![1], vec![0, 2], vec![1]];
        assert!(!is_cycle_undirected(3, &adj2));
    }
}
