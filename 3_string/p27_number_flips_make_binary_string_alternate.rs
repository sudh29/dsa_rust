pub fn min_flips(s: &str) -> usize {
    let mut flips1 = 0; // expected: 0, 1, 0, 1...
    let mut flips2 = 0; // expected: 1, 0, 1, 0...

    for (i, c) in s.chars().enumerate() {
        let expected1 = if i % 2 == 0 { '0' } else { '1' };
        let expected2 = if i % 2 == 0 { '1' } else { '0' };
        if c != expected1 {
            flips1 += 1;
        }
        if c != expected2 {
            flips2 += 1;
        }
    }
    flips1.min(flips2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_flips() {
        assert_eq!(min_flips("001"), 1);
        assert_eq!(min_flips("0001010111"), 2);
    }
}
