pub fn all_possible_strings(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let total = 1 << n;
    let mut res = Vec::new();

    for i in 1..total {
        let mut sub = String::new();
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                sub.push(chars[j]);
            }
        }
        res.push(sub);
    }
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_set() {
        let ps = all_possible_strings("abc");
        assert_eq!(ps.len(), 7);
        assert_eq!(ps[0], "a");
        assert_eq!(ps[6], "c");
    }
}
