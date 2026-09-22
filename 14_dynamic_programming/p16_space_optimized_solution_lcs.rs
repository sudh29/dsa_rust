pub fn lcs_space_optimized(x: &str, y: &str) -> usize {
    let (b1, b2) = (x.as_bytes(), y.as_bytes());
    let (m, n) = (b1.len(), b2.len());
    let mut prev = vec![0; n + 1];
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        for j in 1..=n {
            if b1[i - 1] == b2[j - 1] {
                curr[j] = prev[j - 1] + 1;
            } else {
                curr[j] = prev[j].max(curr[j - 1]);
            }
        }
        prev.copy_from_slice(&curr);
    }
    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_space_opt() {
        assert_eq!(lcs_space_optimized("AGGTAB", "GXTXAYB"), 4);
    }
}
