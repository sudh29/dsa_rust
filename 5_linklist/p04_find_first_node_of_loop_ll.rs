pub fn detect_loop_start(next_indices: &[Option<usize>]) -> Option<usize> {
    let mut visited = vec![false; next_indices.len()];
    let mut curr = Some(0);

    while let Some(idx) = curr {
        if visited[idx] {
            return Some(idx);
        }
        visited[idx] = true;
        curr = next_indices[idx];
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_start() {
        let graph = vec![Some(1), Some(2), Some(3), Some(1)];
        assert_eq!(detect_loop_start(&graph), Some(1));
    }
}
