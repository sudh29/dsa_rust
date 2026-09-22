pub fn water_connection(n: usize, pipes: &[[usize; 3]]) -> Vec<[usize; 3]> {
    let mut to_node = vec![0; n + 1];
    let mut from_node = vec![0; n + 1];
    let mut diameter = vec![0; n + 1];

    for &[u, v, d] in pipes {
        to_node[u] = v;
        from_node[v] = u;
        diameter[u] = d;
    }

    let mut res = Vec::new();
    for i in 1..=n {
        if from_node[i] == 0 && to_node[i] != 0 {
            let mut curr = i;
            let mut min_dia = usize::MAX;
            while to_node[curr] != 0 {
                min_dia = min_dia.min(diameter[curr]);
                curr = to_node[curr];
            }
            res.push([i, curr, min_dia]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_connection() {
        let pipes = [
            [7, 4, 98],
            [5, 9, 72],
            [4, 6, 10],
            [2, 8, 22],
            [9, 7, 17],
            [3, 1, 66],
        ];
        let res = water_connection(9, &pipes);
        assert!(!res.is_empty());
    }
}
