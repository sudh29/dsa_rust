pub fn overlapped_interval(intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    crate::array::p13_merge_intervals::merge_intervals(intervals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapped() {
        let intvs = vec![[1, 3], [2, 4], [6, 8], [9, 10]];
        assert_eq!(overlapped_interval(intvs), vec![[1, 4], [6, 8], [9, 10]]);
    }
}
