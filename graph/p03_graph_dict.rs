use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct GraphDict {
    pub graph: HashMap<i32, Vec<i32>>,
}

impl GraphDict {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, u: i32, v: i32) {
        self.graph.entry(u).or_default().push(v);
    }

    pub fn get_neighbors(&self, u: i32) -> &[i32] {
        self.graph.get(&u).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_dict() {
        let mut g = GraphDict::new();
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        assert_eq!(g.get_neighbors(0), &[1, 2]);
    }
}
