pub fn max_meetings(start: &[i32], end: &[i32]) -> usize {
    crate::greedy::p00_activity_selection_problem::activity_selection(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meetings() {
        let start = [1, 3, 0, 5, 8, 5];
        let end = [2, 4, 6, 7, 9, 9];
        assert_eq!(max_meetings(&start, &end), 4);
    }
}
