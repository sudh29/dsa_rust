const MOD: i64 = 1_000_000_007;

pub fn count_ways(n: usize, k: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return k % MOD;
    }
    let mut same = k % MOD;
    let mut diff = (k * (k - 1)) % MOD;
    let mut total = (same + diff) % MOD;
    for _ in 3..=n {
        same = diff;
        diff = (total * (k - 1)) % MOD;
        total = (same + diff) % MOD;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_painting_fence() {
        assert_eq!(count_ways(3, 2), 6);
        assert_eq!(count_ways(2, 4), 16);
    }
}
