use std::collections::HashMap;

pub struct ConceptGraph {
    pub nodes: Vec<String>,
    pub edges: HashMap<(usize, usize), f64>,
}

impl ConceptGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), edges: HashMap::new() }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, weight: f64) {
        self.edges.insert((a, b), weight);
    }
}
