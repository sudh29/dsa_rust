pub fn print_graph(v: usize, edges: &[[usize; 2]]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); v];
    for edge in edges {
        let (u, w) = (edge[0], edge[1]);
        adj[u].push(w);
        adj[w].push(u);
    }
    adj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_graph() {
        let edges = [[0, 1], [0, 4], [4, 1], [4, 3], [1, 3], [1, 2], [3, 2]];
        let adj = print_graph(5, &edges);
        assert_eq!(adj[0], vec![1, 4]);
        assert_eq!(adj[4], vec![0, 1, 3]);
    }
}
