pub fn min_cost_cut_board(mut x: Vec<i64>, mut y: Vec<i64>) -> i64 {
    x.sort_by(|a, b| b.cmp(a));
    y.sort_by(|a, b| b.cmp(a));

    let mut hz_pieces = 1;
    let mut vt_pieces = 1;
    let (mut i, mut j) = (0, 0);
    let mut total_cost = 0;

    while i < x.len() && j < y.len() {
        if x[i] >= y[j] {
            total_cost += x[i] * hz_pieces;
            vt_pieces += 1;
            i += 1;
        } else {
            total_cost += y[j] * vt_pieces;
            hz_pieces += 1;
            j += 1;
        }
    }
    while i < x.len() {
        total_cost += x[i] * hz_pieces;
        i += 1;
    }
    while j < y.len() {
        total_cost += y[j] * vt_pieces;
        j += 1;
    }
    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_cut() {
        assert_eq!(min_cost_cut_board(vec![2, 1, 3, 1, 4], vec![4, 1, 2]), 42);
    }
}
