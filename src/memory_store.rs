use crate::cluster::ClusterLayer;

pub struct MemoryStore {
    examples: Vec<f64>,
}

impl MemoryStore {
    pub fn new() -> Self { Self { examples: Vec::new() } }
    pub fn update(&mut self, clusters: &ClusterLayer) {
        for c in &clusters.clusters {
            self.examples.extend(c.center.iter().cloned());
        }
    }
}
