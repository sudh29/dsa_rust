pub fn min_swaps_bracket_balancing(s: &str) -> usize {
    let mut count = 0;
    let mut p = 0;
    let mut swaps = 0;

    for c in s.chars() {
        if c == '[' {
            count += 1;
            if p > 0 {
                swaps += p;
                p -= 1;
            }
        } else if c == ']' {
            count -= 1;
            if count < 0 {
                p -= count;
            }
        }
    }
    swaps as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracket_swaps() {
        assert_eq!(min_swaps_bracket_balancing("[]][]["), 2);
        assert_eq!(min_swaps_bracket_balancing("[[][]]"), 0);
    }
}
