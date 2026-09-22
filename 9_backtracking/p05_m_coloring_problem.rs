pub fn graph_coloring(adj: &[Vec<usize>], m: usize) -> bool {
    let n = adj.len();
    let mut colors = vec![0; n];

    fn is_safe(node: usize, c: usize, adj: &[Vec<usize>], colors: &[usize]) -> bool {
        for &neighbor in &adj[node] {
            if colors[neighbor] == c {
                return false;
            }
        }
        true
    }

    fn solve(node: usize, n: usize, m: usize, adj: &[Vec<usize>], colors: &mut [usize]) -> bool {
        if node == n {
            return true;
        }
        for c in 1..=m {
            if is_safe(node, c, adj, colors) {
                colors[node] = c;
                if solve(node + 1, n, m, adj, colors) {
                    return true;
                }
                colors[node] = 0;
            }
        }
        false
    }

    solve(0, n, m, adj, &mut colors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coloring() {
        let adj = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert!(graph_coloring(&adj, 3));
        assert!(!graph_coloring(&adj, 2));
    }
}
