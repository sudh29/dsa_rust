use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct WeightedGraph {
    pub adj_list: HashMap<String, Vec<(String, i32)>>,
}

impl WeightedGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, u: &str, v: &str, dist: i32, bidirectional: bool) {
        self.adj_list
            .entry(u.to_string())
            .or_default()
            .push((v.to_string(), dist));
        if bidirectional {
            self.adj_list
                .entry(v.to_string())
                .or_default()
                .push((u.to_string(), dist));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph_print() {
        let mut g = WeightedGraph::new();
        g.add_edge("A", "B", 10, true);
        g.add_edge("B", "C", 20, true);
        assert_eq!(g.adj_list.get("A").unwrap().len(), 1);
        assert_eq!(g.adj_list.get("B").unwrap().len(), 2);
    }
}
