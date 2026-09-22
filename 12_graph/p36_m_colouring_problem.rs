pub fn graph_coloring(graph: &[Vec<bool>], m: usize, v: usize) -> bool {
    let mut color = vec![0; v];

    fn is_valid(node: usize, c: usize, graph: &[Vec<bool>], color: &[usize], v: usize) -> bool {
        for i in 0..v {
            if graph[node][i] && color[i] == c {
                return false;
            }
        }
        true
    }

    fn solve(node: usize, m: usize, graph: &[Vec<bool>], color: &mut [usize], v: usize) -> bool {
        if node == v {
            return true;
        }
        for c in 1..=m {
            if is_valid(node, c, graph, color, v) {
                color[node] = c;
                if solve(node + 1, m, graph, color, v) {
                    return true;
                }
                color[node] = 0;
            }
        }
        false
    }

    solve(0, m, graph, &mut color, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coloring() {
        let graph = vec![
            vec![false, true, true, true],
            vec![true, false, true, false],
            vec![true, true, false, true],
            vec![true, false, true, false],
        ];
        assert!(graph_coloring(&graph, 3, 4));
        assert!(!graph_coloring(&graph, 2, 4));
    }
}
