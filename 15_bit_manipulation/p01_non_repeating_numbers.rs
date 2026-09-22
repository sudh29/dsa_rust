pub fn single_number(nums: &[i32]) -> (i32, i32) {
    let mut xor_all = 0;
    for &x in nums {
        xor_all ^= x;
    }
    let diff_bit = xor_all.isolate_lowest_one();
    let mut a = 0;
    let mut b = 0;
    for &x in nums {
        if (x & diff_bit) != 0 {
            a ^= x;
        } else {
            b ^= x;
        }
    }
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(&[1, 2, 3, 2, 1, 4]), (3, 4));
        assert_eq!(single_number(&[2, 1, 3, 2]), (1, 3));
    }
}
