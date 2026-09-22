pub fn activity_selection(start: &[i32], end: &[i32]) -> usize {
    let mut activities: Vec<(i32, i32)> = start.iter().copied().zip(end.iter().copied()).collect();
    activities.sort_by_key(|x| x.1); // sort by end time

    let mut count = 0;
    let mut last_end = -1;

    for (s, e) in activities {
        if s > last_end {
            count += 1;
            last_end = e;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_selection() {
        let start = [1, 3, 2, 5];
        let end = [2, 4, 3, 6];
        assert_eq!(activity_selection(&start, &end), 3);
    }
}
