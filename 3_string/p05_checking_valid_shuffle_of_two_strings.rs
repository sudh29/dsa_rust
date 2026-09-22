pub fn valid_shuffle(s1: &str, s2: &str, shuffle: &str) -> bool {
    if s1.len() + s2.len() != shuffle.len() {
        return false;
    }
    let (b1, b2, bs) = (s1.as_bytes(), s2.as_bytes(), shuffle.as_bytes());
    let (mut i, mut j) = (0, 0);

    for &c in bs {
        if i < b1.len() && b1[i] == c {
            i += 1;
        } else if j < b2.len() && b2[j] == c {
            j += 1;
        } else {
            return false;
        }
    }
    i == b1.len() && j == b2.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        assert!(valid_shuffle("abc", "def", "dabecf"));
        assert!(!valid_shuffle("abc", "def", "defbca"));
    }
}
