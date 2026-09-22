pub fn merge_intervals(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    if intervals.len() <= 1 {
        return intervals;
    }
    intervals.sort_by_key(|x| x[0]);
    let mut merged: Vec<[i32; 2]> = Vec::new();

    for interval in intervals {
        if let Some(last) = merged.last_mut() {
            if interval[0] <= last[1] {
                last[1] = last[1].max(interval[1]);
            } else {
                merged.push(interval);
            }
        } else {
            merged.push(interval);
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        let intervals = vec![[1, 3], [2, 6], [8, 10], [15, 18]];
        assert_eq!(merge_intervals(intervals), vec![[1, 6], [8, 10], [15, 18]]);
    }
}
