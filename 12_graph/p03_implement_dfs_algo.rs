pub fn dfs_of_graph(v: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    if v == 0 {
        return Vec::new();
    }
    let mut visited = vec![false; v];
    let mut res = Vec::new();

    fn dfs(u: usize, adj: &[Vec<usize>], visited: &mut [bool], res: &mut Vec<usize>) {
        visited[u] = true;
        res.push(u);
        for &neighbor in &adj[u] {
            if !visited[neighbor] {
                dfs(neighbor, adj, visited, res);
            }
        }
    }

    dfs(0, adj, &mut visited, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1, 3, 4], vec![2], vec![2]];
        assert_eq!(dfs_of_graph(5, &adj), vec![0, 1, 2, 3, 4]);
    }
}
