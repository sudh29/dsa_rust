pub fn find_max_product_subset(arr: &[i64]) -> i64 {
    let modulo = 1_000_000_007i64;
    if arr.len() == 1 {
        return arr[0];
    }
    let mut max_neg = i64::MIN;
    let mut neg_count = 0;
    let mut zero_count = 0;
    let mut prod = 1i64;

    for &x in arr {
        if x == 0 {
            zero_count += 1;
            continue;
        }
        if x < 0 {
            neg_count += 1;
            max_neg = max_neg.max(x);
        }
        prod = (prod * x.abs()) % modulo;
    }

    if zero_count == arr.len() || (neg_count == 1 && zero_count + 1 == arr.len()) {
        return 0;
    }
    if neg_count % 2 == 1 {
        // divide out max_neg modulo
        let mut actual_prod = 1i64;
        let mut skipped = false;
        for &x in arr {
            if x == 0 {
                continue;
            }
            if x == max_neg && !skipped {
                skipped = true;
                continue;
            }
            actual_prod = (actual_prod * x) % modulo;
        }
        (actual_prod + modulo) % modulo
    } else {
        (prod + modulo) % modulo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_prod_subset() {
        assert_eq!(find_max_product_subset(&[-1, -1, -2, 4, 3]), 24);
        assert_eq!(find_max_product_subset(&[-1, 0]), 0);
    }
}
