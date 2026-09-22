use std::collections::HashSet;

pub fn do_union(a: &[i32], b: &[i32]) -> usize {
    let mut set = HashSet::new();
    for &x in a {
        set.insert(x);
    }
    for &x in b {
        set.insert(x);
    }
    set.len()
}

pub fn intersect(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut a = nums1.to_vec();
    let mut b = nums2.to_vec();
    a.sort_unstable();
    b.sort_unstable();
    let (mut i, mut j) = (0, 0);
    let mut res = Vec::new();
    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            res.push(a[i]);
            i += 1;
            j += 1;
        } else if a[i] < b[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        assert_eq!(do_union(&[1, 2, 3, 4, 5], &[1, 2, 3]), 5);
        assert_eq!(do_union(&[85, 25, 1, 32, 54, 6], &[85, 2]), 7);
    }

    #[test]
    fn test_intersect() {
        assert_eq!(intersect(&[1, 2, 2, 1], &[2, 2]), vec![2, 2]);
    }
}
