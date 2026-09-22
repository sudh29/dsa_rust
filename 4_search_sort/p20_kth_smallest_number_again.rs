pub fn kth_smallest_in_intervals(mut intervals: Vec<[i64; 2]>, k: i64) -> i64 {
    intervals.sort_by_key(|x| x[0]);
    let mut merged: Vec<[i64; 2]> = Vec::new();

    for intv in intervals {
        if let Some(last) = merged.last_mut() {
            if intv[0] <= last[1] {
                last[1] = last[1].max(intv[1]);
            } else {
                merged.push(intv);
            }
        } else {
            merged.push(intv);
        }
    }

    let mut remaining = k;
    for intv in merged {
        let count = intv[1] - intv[0] + 1;
        if remaining <= count {
            return intv[0] + remaining - 1;
        }
        remaining -= count;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_interval() {
        let intervals = vec![[1, 5], [10, 15]];
        assert_eq!(kth_smallest_in_intervals(intervals.clone(), 3), 3);
        assert_eq!(kth_smallest_in_intervals(intervals, 6), 10);
    }
}
