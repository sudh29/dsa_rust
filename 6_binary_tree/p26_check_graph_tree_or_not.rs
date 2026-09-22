pub fn is_tree(n: usize, edges: &[[usize; 2]]) -> bool {
    if edges.len() != n.saturating_sub(1) {
        return false;
    }
    let mut adj = vec![Vec::new(); n];
    for &[u, v] in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut visited = vec![false; n];
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], visited: &mut [bool]) -> bool {
        visited[u] = true;
        for &v in &adj[u] {
            if v == p {
                continue;
            }
            if visited[v] {
                return false;
            }
            if !dfs(v, u, adj, visited) {
                return false;
            }
        }
        true
    }
    if !dfs(0, usize::MAX, &adj, &mut visited) {
        return false;
    }
    visited.into_iter().all(|x| x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tree() {
        assert!(is_tree(5, &[[0, 1], [0, 2], [0, 3], [1, 4]]));
        assert!(!is_tree(5, &[[0, 1], [1, 2], [2, 0], [1, 3], [1, 4]]));
    }
}
