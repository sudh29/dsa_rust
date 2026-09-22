pub fn common_elements(a: &[i32], b: &[i32], c: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut res = Vec::new();

    while i < a.len() && j < b.len() && k < c.len() {
        if a[i] == b[j] && b[j] == c[k] {
            if res.is_empty() || *res.last().unwrap() != a[i] {
                res.push(a[i]);
            }
            i += 1;
            j += 1;
            k += 1;
        } else if a[i] < b[j] {
            i += 1;
        } else if b[j] < c[k] {
            j += 1;
        } else {
            k += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_elements() {
        let a = [1, 5, 10, 20, 40, 80];
        let b = [6, 7, 20, 80, 100];
        let c = [3, 4, 15, 20, 30, 70, 80, 120];
        assert_eq!(common_elements(&a, &b, &c), vec![20, 80]);
    }
}
