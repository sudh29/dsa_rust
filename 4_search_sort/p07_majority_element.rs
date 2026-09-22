pub fn majority_element(arr: &[i32]) -> i32 {
    let mut count = 0;
    let mut candidate = 0;

    for &num in arr {
        if count == 0 {
            candidate = num;
        }
        if num == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    let actual_count = arr.iter().filter(|&&x| x == candidate).count();
    if actual_count > arr.len() / 2 {
        candidate
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority() {
        assert_eq!(majority_element(&[3, 1, 3, 3, 2]), 3);
        assert_eq!(majority_element(&[1, 2, 3]), -1);
    }
}
