pub fn count_ones(n: usize) -> Vec<u32> {
    let mut res = Vec::with_capacity(n + 1);
    for i in 0..=n {
        res.push((i as u32).count_ones());
    }
    res
}

pub fn count_ones_dp(n: usize) -> Vec<u32> {
    let mut res = vec![0; n + 1];
    for i in 1..=n {
        res[i] = res[i / 2] + (i % 2) as u32;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ones() {
        assert_eq!(count_ones(5), vec![0, 1, 1, 2, 1, 2]);
        assert_eq!(count_ones_dp(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
