pub fn kth_element_two_arrays(arr1: &[i32], arr2: &[i32], k: usize) -> i32 {
    let (mut i, mut j) = (0, 0);
    let mut count = 0;

    while i < arr1.len() && j < arr2.len() {
        let val;
        if arr1[i] < arr2[j] {
            val = arr1[i];
            i += 1;
        } else {
            val = arr2[j];
            j += 1;
        }
        count += 1;
        if count == k {
            return val;
        }
    }
    while i < arr1.len() {
        count += 1;
        if count == k {
            return arr1[i];
        }
        i += 1;
    }
    while j < arr2.len() {
        count += 1;
        if count == k {
            return arr2[j];
        }
        j += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_element() {
        let a = [2, 3, 6, 7, 9];
        let b = [1, 4, 8, 10];
        assert_eq!(kth_element_two_arrays(&a, &b, 5), 6);
    }
}
