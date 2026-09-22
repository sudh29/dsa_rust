use std::collections::HashMap;

pub fn check_mirror_tree(n: usize, edges1: &[[usize; 2]], edges2: &[[usize; 2]]) -> bool {
    let mut map1: HashMap<usize, Vec<usize>> = HashMap::new();
    for &[u, v] in edges1 {
        map1.entry(u).or_default().push(v);
    }
    let mut map2: HashMap<usize, Vec<usize>> = HashMap::new();
    for &[u, v] in edges2 {
        map2.entry(u).or_default().push(v);
    }
    for (u, list1) in map1 {
        if let Some(list2) = map2.get(&u) {
            let rev2: Vec<usize> = list2.iter().copied().rev().collect();
            if list1 != rev2 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_ary_mirror() {
        let e1 = [[1, 2], [1, 3]];
        let e2 = [[1, 3], [1, 2]];
        assert!(check_mirror_tree(3, &e1, &e2));
    }
}
