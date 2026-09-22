pub fn job_scheduling(deadlines: &[i32], profits: &[i32]) -> (usize, i32) {
    let mut jobs: Vec<(i32, i32)> = deadlines
        .iter()
        .copied()
        .zip(profits.iter().copied())
        .collect();
    jobs.sort_by(|a, b| b.1.cmp(&a.1)); // highest profit first

    let max_deadline = deadlines.iter().copied().max().unwrap_or(0) as usize;
    let mut slot = vec![false; max_deadline + 1];
    let mut count = 0;
    let mut total_profit = 0;

    for (d, p) in jobs {
        for t in (1..=d as usize).rev() {
            if !slot[t] {
                slot[t] = true;
                count += 1;
                total_profit += p;
                break;
            }
        }
    }
    (count, total_profit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_sequencing() {
        let deadlines = [4, 1, 1, 1];
        let profits = [20, 10, 40, 30];
        assert_eq!(job_scheduling(&deadlines, &profits), (2, 60));
    }
}
