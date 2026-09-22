pub fn min_swaps_to_bst(arr: &[i32]) -> usize {
    let mut inorder = Vec::new();
    fn inorder_traversal(arr: &[i32], idx: usize, res: &mut Vec<i32>) {
        if idx >= arr.len() {
            return;
        }
        inorder_traversal(arr, 2 * idx + 1, res);
        res.push(arr[idx]);
        inorder_traversal(arr, 2 * idx + 2, res);
    }
    inorder_traversal(arr, 0, &mut inorder);
    crate::search_sort::p17_minimum_swaps_to_sort::min_swaps(&inorder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps_bst() {
        let tree_arr = [5, 6, 7, 8, 9, 10, 11];
        assert_eq!(min_swaps_to_bst(&tree_arr), 3);
    }
}
