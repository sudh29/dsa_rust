pub fn is_cyclic_directed(v: usize, adj: &[Vec<usize>]) -> bool {
    let mut visited = vec![false; v];
    let mut rec_stack = vec![false; v];

    fn dfs(u: usize, adj: &[Vec<usize>], visited: &mut [bool], rec_stack: &mut [bool]) -> bool {
        visited[u] = true;
        rec_stack[u] = true;

        for &neighbor in &adj[u] {
            if !visited[neighbor] {
                if dfs(neighbor, adj, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack[neighbor] {
                return true;
            }
        }

        rec_stack[u] = false;
        false
    }

    for i in 0..v {
        if !visited[i] && dfs(i, adj, &mut visited, &mut rec_stack) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_directed() {
        let adj = vec![vec![1], vec![2], vec![0], vec![2]];
        assert!(is_cyclic_directed(4, &adj));

        let adj2 = vec![vec![1], vec![2], vec![]];
        assert!(!is_cyclic_directed(3, &adj2));
    }
}
