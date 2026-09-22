#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub num_vertices: usize,
    pub adj: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Graph {
            num_vertices,
            adj: vec![Vec::new(); num_vertices],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
        if u < self.num_vertices && v < self.num_vertices {
            self.adj[u].push((v, weight));
        }
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, weight: i32) {
        self.add_edge(u, v, weight);
        self.add_edge(v, u, weight);
    }
}
