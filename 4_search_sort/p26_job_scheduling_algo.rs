pub fn job_scheduling(start_time: &[i32], end_time: &[i32], profit: &[i32]) -> i32 {
    let mut jobs: Vec<(i32, i32, i32)> = start_time
        .iter()
        .copied()
        .zip(end_time.iter().copied())
        .zip(profit.iter().copied())
        .map(|((s, e), p)| (s, e, p))
        .collect();
    jobs.sort_by_key(|x| x.1); // sort by end time

    let n = jobs.len();
    let mut dp = vec![0; n];
    dp[0] = jobs[0].2;

    for i in 1..n {
        let curr_profit = jobs[i].2;
        let mut prev = -1;
        // binary search for latest non-conflicting job
        let mut low = 0;
        let mut high = (i - 1) as isize;
        while low <= high {
            let mid = low + (high - low) / 2;
            if jobs[mid as usize].1 <= jobs[i].0 {
                prev = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        let with_job = if prev != -1 {
            curr_profit + dp[prev as usize]
        } else {
            curr_profit
        };
        dp[i] = dp[i - 1].max(with_job);
    }
    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_scheduling() {
        assert_eq!(
            job_scheduling(&[1, 2, 3, 3], &[3, 4, 5, 6], &[50, 10, 40, 70]),
            120
        );
    }
}
