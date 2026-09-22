pub fn find_conflicts(appointments: &[[i32; 2]]) -> Vec<([i32; 2], [i32; 2])> {
    let mut conflicts = Vec::new();
    let n = appointments.len();
    for i in 0..n {
        for j in i + 1..n {
            if appointments[i][0] < appointments[j][1] && appointments[j][0] < appointments[i][1] {
                conflicts.push((appointments[i], appointments[j]));
            }
        }
    }
    conflicts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflicts() {
        let appts = [[1, 5], [3, 7], [2, 6], [10, 15], [5, 6], [4, 100]];
        let conflicts = find_conflicts(&appts);
        assert!(!conflicts.is_empty());
    }
}
