pub fn unique_paths(m: usize, n: usize) -> usize {
    crate::backtracking::p16_print_all_possible_paths_from_top_left_bottom_right_mxn_matrix::all_paths_grid(m, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        assert_eq!(unique_paths(3, 7), 28);
    }
}
