pub fn rabin_karp(pattern: &str, text: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    let p_len = pattern.len();
    let t_len = text.len();
    if p_len == 0 || p_len > t_len {
        return matches;
    }

    let p_bytes = pattern.as_bytes();
    let t_bytes = text.as_bytes();

    let prime: u64 = 101;
    let base: u64 = 256;
    let mut h: u64 = 1;

    for _ in 0..p_len - 1 {
        h = (h * base) % prime;
    }

    let mut p_hash: u64 = 0;
    let mut t_hash: u64 = 0;

    for i in 0..p_len {
        p_hash = (base * p_hash + p_bytes[i] as u64) % prime;
        t_hash = (base * t_hash + t_bytes[i] as u64) % prime;
    }

    for i in 0..=t_len - p_len {
        if p_hash == t_hash && &t_bytes[i..i + p_len] == p_bytes {
            matches.push(i);
        }
        if i < t_len - p_len {
            t_hash = (base * (t_hash + prime - (t_bytes[i] as u64 * h) % prime)
                + t_bytes[i + p_len] as u64)
                % prime;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rabin_karp() {
        assert_eq!(rabin_karp("AABA", "AABAACAADAABAABA"), vec![0, 9, 12]);
    }
}
