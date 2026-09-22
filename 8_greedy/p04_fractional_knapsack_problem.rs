pub fn fractional_knapsack(w: usize, values: &[f64], weights: &[f64]) -> f64 {
    let mut items: Vec<(f64, f64)> = values
        .iter()
        .copied()
        .zip(weights.iter().copied())
        .collect();
    items.sort_by(|a, b| (b.0 / b.1).partial_cmp(&(a.0 / a.1)).unwrap());

    let mut current_weight = 0.0;
    let mut total_val = 0.0;
    let w = w as f64;

    for (v, wt) in items {
        if current_weight + wt <= w {
            current_weight += wt;
            total_val += v;
        } else {
            let remain = w - current_weight;
            total_val += v * (remain / wt);
            break;
        }
    }
    total_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractional_knapsack() {
        let values = [60.0, 100.0, 120.0];
        let weights = [10.0, 20.0, 30.0];
        assert_eq!(fractional_knapsack(50, &values, &weights), 240.0);
    }
}
