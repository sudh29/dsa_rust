pub fn candy_store(mut candies: Vec<i32>, k: usize) -> (i32, i32) {
    candies.sort_unstable();
    let n = candies.len();

    // Min cost
    let mut min_cost = 0;
    let mut i = 0;
    let mut end = n;
    while i < end {
        min_cost += candies[i];
        i += 1;
        end = end.saturating_sub(k);
    }

    // Max cost
    let mut max_cost = 0;
    let mut idx = n;
    let mut start = 0;
    while idx > start {
        idx -= 1;
        max_cost += candies[idx];
        start += k;
    }

    (min_cost, max_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy_store() {
        assert_eq!(candy_store(vec![3, 2, 1, 4], 2), (3, 7));
    }
}
