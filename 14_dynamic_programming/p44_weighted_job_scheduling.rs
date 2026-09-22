#[derive(Debug, Clone)]
pub struct Job {
    pub id: i32,
    pub deadline: usize,
    pub profit: i32,
}

pub fn job_scheduling(mut jobs: Vec<Job>) -> (usize, i32) {
    jobs.sort_by(|a, b| b.profit.cmp(&a.profit));
    let max_deadline = jobs.iter().map(|j| j.deadline).max().unwrap_or(0);
    let mut slot = vec![-1; max_deadline + 1];
    let mut count = 0;
    let mut total_profit = 0;
    for job in jobs {
        for j in (1..=job.deadline.min(max_deadline)).rev() {
            if slot[j] == -1 {
                slot[j] = job.id;
                count += 1;
                total_profit += job.profit;
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
    fn test_job_sched() {
        let jobs = vec![
            Job {
                id: 1,
                deadline: 4,
                profit: 20,
            },
            Job {
                id: 2,
                deadline: 1,
                profit: 10,
            },
            Job {
                id: 3,
                deadline: 1,
                profit: 40,
            },
            Job {
                id: 4,
                deadline: 1,
                profit: 30,
            },
        ];
        let (c, p) = job_scheduling(jobs);
        assert_eq!(c, 2);
        assert_eq!(p, 60);
    }
}
