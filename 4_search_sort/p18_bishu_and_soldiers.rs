pub fn bishu_fight(powers: &mut [i32], q_powers: &[i32]) -> Vec<(usize, i32)> {
    powers.sort_unstable();
    let mut prefix_sum = vec![0; powers.len() + 1];
    for (i, &p) in powers.iter().enumerate() {
        prefix_sum[i + 1] = prefix_sum[i] + p;
    }
    let mut res = Vec::new();
    for &p in q_powers {
        let idx = match powers.binary_search(&p) {
            Ok(mut pos) => {
                while pos < powers.len() && powers[pos] == p {
                    pos += 1;
                }
                pos
            }
            Err(pos) => pos,
        };
        res.push((idx, prefix_sum[idx]));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bishu() {
        let mut powers = [1, 2, 3, 4, 5, 6, 7];
        let queries = [3, 10, 2];
        assert_eq!(
            bishu_fight(&mut powers, &queries),
            vec![(3, 6), (7, 28), (2, 3)]
        );
    }
}
