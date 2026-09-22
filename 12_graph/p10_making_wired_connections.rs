pub fn make_connected(n: usize, connections: &[[usize; 2]]) -> i32 {
    let m = connections.len();
    if m < n - 1 {
        return -1;
    }
    let mut adj = vec![Vec::new(); n];
    for &[u, v] in connections {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut visited = vec![false; n];
    let mut components = 0;

    fn dfs(u: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
        visited[u] = true;
        for &neighbor in &adj[u] {
            if !visited[neighbor] {
                dfs(neighbor, adj, visited);
            }
        }
    }

    for i in 0..n {
        if !visited[i] {
            components += 1;
            dfs(i, &adj, &mut visited);
        }
    }
    components - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connections() {
        let conn = [[0, 1], [0, 2], [1, 2]];
        assert_eq!(make_connected(4, &conn), 1);
        let conn2 = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]];
        assert_eq!(make_connected(6, &conn2), 2);
        let conn3 = [[0, 1], [0, 2], [0, 3], [1, 2]];
        assert_eq!(make_connected(6, &conn3), -1);
    }
}
