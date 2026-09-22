pub fn merge_sorted(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            res.push(arr1[i]);
            i += 1;
        } else {
            res.push(arr2[j]);
            j += 1;
        }
    }
    while i < arr1.len() {
        res.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        res.push(arr2[j]);
        j += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted() {
        assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }
}
