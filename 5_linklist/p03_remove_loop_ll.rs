pub fn remove_loop_in_indices(next_indices: &mut [Option<usize>]) -> bool {
    let mut visited = vec![false; next_indices.len()];
    let mut curr = Some(0);
    let mut prev = None;

    while let Some(idx) = curr {
        if visited[idx] {
            if let Some(p) = prev {
                next_indices[p] = None;
                return true;
            }
        }
        visited[idx] = true;
        prev = Some(idx);
        curr = next_indices[idx];
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_loop() {
        let mut loop_graph = vec![Some(1), Some(2), Some(0)];
        assert!(remove_loop_in_indices(&mut loop_graph));
        assert_eq!(loop_graph[2], None);
    }
}
