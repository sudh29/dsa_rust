/// Computes the Longest Proper Prefix which is also Suffix (LPS) array for KMP matching.
///
/// # Complexity
/// - Time Complexity: $O(M)$ where $M$ is the pattern length
/// - Space Complexity: $O(M)$
pub fn compute_lps(pattern: &str) -> Vec<usize> {
    let b = pattern.as_bytes();
    let n = b.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    let mut i = 1;

    while i < n {
        if b[i] == b[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}

/// Searches for all starting indices of `pattern` in `text` using Knuth-Morris-Pratt (KMP).
///
/// # Complexity
/// - Time Complexity: $O(N + M)$
/// - Space Complexity: $O(M)$ auxiliary space for LPS table
///
/// # Examples
/// ```
/// use dsa_rust::string::p18_kmp_algo::kmp_search;
///
/// let matches = kmp_search("ABABCABAB", "ABABDABACDABABCABAB");
/// assert_eq!(matches, vec![10]);
/// ```
pub fn kmp_search(pattern: &str, text: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    let p = pattern.as_bytes();
    let t = text.as_bytes();
    if p.is_empty() || p.len() > t.len() {
        return matches;
    }

    let lps = compute_lps(pattern);
    let mut i = 0;
    let mut j = 0;

    while i < t.len() {
        if p[j] == t[i] {
            i += 1;
            j += 1;
        }
        if j == p.len() {
            matches.push(i - j);
            j = lps[j - 1];
        } else if i < t.len() && p[j] != t[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp() {
        assert_eq!(kmp_search("ABABCABAB", "ABABDABACDABABCABAB"), vec![10]);
        assert_eq!(kmp_search("A", "AAAAA"), vec![0, 1, 2, 3, 4]);
        assert!(kmp_search("NOTFOUND", "TESTSTRING").is_empty());
        assert!(kmp_search("", "TEXT").is_empty());
    }
}
