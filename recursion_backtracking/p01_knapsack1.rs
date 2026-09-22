pub fn knapsack_recursive(w: usize, val: &[i32], wt: &[usize], n: usize) -> i32 {
    if n == 0 || w == 0 {
        return 0;
    }
    if wt[n - 1] > w {
        knapsack_recursive(w, val, wt, n - 1)
    } else {
        let include = val[n - 1] + knapsack_recursive(w - wt[n - 1], val, wt, n - 1);
        let exclude = knapsack_recursive(w, val, wt, n - 1);
        include.max(exclude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack_rec() {
        let val = [60, 100, 120];
        let wt = [10, 20, 30];
        assert_eq!(knapsack_recursive(50, &val, &wt, 3), 220);
    }
}
