pub fn path_more_than_k(n: usize, adj: &[Vec<(usize, i32)>], src: usize, k: i32) -> bool {
    let mut visited = vec![false; n];
    visited[src] = true;

    fn dfs(u: usize, k: i32, adj: &[Vec<(usize, i32)>], visited: &mut [bool]) -> bool {
        if k <= 0 {
            return true;
        }
        for &(v, w) in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                if dfs(v, k - w, adj, visited) {
                    return true;
                }
                visited[v] = false;
            }
        }
        false
    }

    dfs(src, k, adj, &mut visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_k() {
        let mut adj = vec![Vec::new(); 4];
        adj[0].push((1, 5));
        adj[1].push((2, 10));
        adj[2].push((3, 20));
        assert!(path_more_than_k(4, &adj, 0, 35));
        assert!(!path_more_than_k(4, &adj, 0, 50));
    }
}
