pub fn get_permutation(n: usize, mut k: usize) -> String {
    let mut numbers: Vec<usize> = (1..=n).collect();
    let mut fact = vec![1; n];
    for i in 1..n {
        fact[i] = fact[i - 1] * i;
    }
    k -= 1; // 0-indexed
    let mut res = String::new();

    for i in (0..n).rev() {
        let idx = k / fact[i];
        res.push_str(&numbers.remove(idx).to_string());
        k %= fact[i];
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_perm() {
        assert_eq!(get_permutation(3, 3), "213");
        assert_eq!(get_permutation(4, 9), "2314");
    }
}
